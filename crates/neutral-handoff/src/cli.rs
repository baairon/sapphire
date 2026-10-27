//! The command line of `sapphire`.
//!
//! ```text
//! sapphire INPUT [EFFECT] [flags]        run an effect and/or stage commands over a video
//! sapphire stage list [--json]           the stages of this tool
//! sapphire stage apply STAGE [flags]     raw frames on stdin, raw frames on stdout
//! ```
//!
//! The wrapper takes the flags of `tools/apply.py` (`--param`, `--layer`, `--frames`, `--depth`,
//! `--lossless`, `--vfx`, `-o`, `--force`, `--dry-run`, `--json`, `--ffmpeg-path`,
//! `--ffprobe-path`) and adds `--stage CMD` (repeatable) and `--stage-timeout SECONDS`.
//! Failures print `error: <code>: <message>` on standard error (see [`crate::error`] for the codes
//! and exit codes) and, with `--json`, the error object on standard output.

use std::io::{Read, Write};

use crate::args::{bad_value, Cursor, Token};
use crate::error::{Contract, Error, Result, EXIT_OK};
use crate::format::{Depth, FrameSpec};
use crate::plan::{prepare, Request};
use crate::rate::Rate;
use crate::run::{dry_run, emit_text, execute, result_json};
use crate::stage;

/// The text of `--help`.
pub const HELP: &str = "usage: sapphire INPUT [EFFECT] [flags]
       sapphire stage list [--json]
       sapphire stage apply STAGE --width W --height H --depth 8|16|32 [--fps N[/D]] [flags]

Runs ffmpeg decode, `vfx apply EFFECT` and/or stage commands, then ffmpeg encode, over a video,
with the plan, commands and checks of tools/apply.py.

flags of the wrapper:
  --param K=V          effect parameter (repeatable)
  --layer ID=VIDEO     an extra input layer of a temporal effect (repeatable)
  --frames N           output length in frames of a temporal effect
  --depth 8|16|32      bits per channel between ffmpeg and the stages (default 16)
  --lossless           ffv1 in RGB instead of libx264 CRF 18 (depth 8 or 16 only)
  --vfx PATH           the vfx executable
  --stage CMD          a raw frame filter between decode and encode (repeatable, run in order).
                       CMD is split like a shell command; {width} {height} {depth} {fps}
                       {frames} {pix_fmt} are replaced. `vfx` marks where the EFFECT runs
                       (default: first); `@NAME [K=V ...]` runs a stage of this tool in-process
  --stage-timeout S    kill a judged process (decode, a stage command, encode; not vfx) that
                       moved no data for S seconds (default 120, 0 disables)
  --contract C         frames/2 (default) or frames/1: the exit codes and report keys spoken
  --geometry G         after a --stage (frames/2): preserve or declared (the stage answers
                       --describe); another value is refused as unsupported
  --describe           frames/2: print the output geometry of the run and stop
  -o, --output PATH    output path (default: <input>_<effect>.mp4, .mkv with --lossless)
  --force              replace an existing -o path once the new output verifies
  --dry-run            print the plan and commands, write nothing
  --json               print a structured result (and errors) as JSON
  --ffmpeg-path PATH   ffmpeg executable (or MEDIA_FFMPEG_PATH, FFMPEG)
  --ffprobe-path PATH  ffprobe executable (or MEDIA_FFPROBE_PATH, FFPROBE)
  -V, --version        print the version
  -h, --help           print this text

exit codes (frames/2): 0 done, 1 stage_failed, 2 usage, 3 unsupported, 4 bad_input / broken_pipe /
partial_frame, 5 stage_timeout, 6 tool_missing. Under --contract frames/1: 0 done, 1 any failure,
2 a command line the parser rejects.
";

/// The text of `stage --help`.
pub const STAGE_HELP: &str = "usage: sapphire stage list [--json] [--contract frames/1|2]
       sapphire stage apply STAGE --width W --height H --depth 8|16|32 [--fps N[/D]]
                                  [--frames N] [--param K=V]... [--in FILE] [--out FILE] [--check]
                                  [--describe] [--ffmpeg-path PATH] [--json] [--contract frames/1|2]

`stage list --json` prints the stage manifest (contracts/stage-manifest.schema.json). `--describe`
reads no input and prints the output geometry (contracts/describe.schema.json). The stages are
`selftest-invert` and `ffmpeg-filter` (--param graph=G, expect=preserve|declared, unsafe=1 for
filters that read files).

`stage apply` reads whole raw frames (rgba, rgba64le or gbrapf32le by --depth) on stdin, writes the
same number of frames of the same size and format on stdout. --frames N checks the frame count.
A failure ends with `error: <code>: frame N: ...` on stderr (or the error object with --json) and the
exit codes of vfx: 2 usage, 3 unsupported, 4 bad input (a partial frame, a closed pipe).
";

/// Whether the process streams are terminals (`stage apply` refuses to read or write frames on one).
#[derive(Clone, Copy, Debug, Default)]
pub struct Terminals {
    /// Standard input is a terminal.
    pub stdin: bool,
    /// Standard output is a terminal.
    pub stdout: bool,
}

enum Parsed {
    Help,
    Version,
    Run(Box<Request>),
}

fn depth_value(flag: &str, text: &str) -> Result<Depth> {
    match text.trim().parse::<u32>() {
        Ok(bits) => Depth::from_bits(bits).ok_or_else(|| bad_value(flag, &format!("invalid choice: '{}' (choose from 8, 16, 32)", text))),
        Err(_) => Err(bad_value(flag, &format!("invalid int value: '{}'", text))),
    }
}

fn parse_wrapper(args: &[String]) -> Result<Parsed> {
    let mut request = Request::default();
    let mut positionals: Vec<String> = Vec::new();
    let mut cursor = Cursor::new(args);
    while let Some(token) = cursor.next_token() {
        let (flag, inline) = match token {
            Token::Positional(p) => {
                positionals.push(p);
                continue;
            }
            Token::Flag(f) => (f, None),
            Token::FlagValue(f, v) => (f, Some(v)),
        };
        match flag.as_str() {
            "-h" | "--help" => return Ok(Parsed::Help),
            "-V" | "--version" => return Ok(Parsed::Version),
            "--param" => {
                let v = cursor.value(&flag, inline)?;
                if v.split_once('=').map(|(k, _)| k.is_empty()).unwrap_or(true) {
                    return Err(bad_value("--param", "expected K=V, for example speed_percent=40"));
                }
                request.params.push(v);
            }
            "--layer" => {
                let v = cursor.value(&flag, inline)?;
                match v.split_once('=') {
                    Some((id, video)) if !id.is_empty() && !video.is_empty() => request.layers.push((id.to_string(), video.to_string())),
                    _ => return Err(bad_value("--layer", "expected ID=VIDEO, for example overlay=overlay.mp4")),
                }
            }
            "--frames" => {
                let v = cursor.value(&flag, inline)?;
                match v.trim().parse::<i64>() {
                    Ok(n) if n >= 1 => request.frames = Some(n as u64),
                    _ => return Err(bad_value("--frames", "expected a positive number of frames")),
                }
            }
            "--depth" => {
                let v = cursor.value(&flag, inline)?;
                request.depth = Some(depth_value("--depth", &v)?);
            }
            "--lossless" => request.lossless = true,
            "--vfx" => request.vfx = Some(cursor.value(&flag, inline)?),
            "-o" | "--output" => request.output = Some(cursor.value(&flag, inline)?),
            "--dry-run" => request.dry_run = true,
            "--json" => request.json = true,
            "--force" => request.force = true,
            "--ffmpeg-path" => request.ffmpeg_path = Some(cursor.value(&flag, inline)?),
            "--ffprobe-path" => request.ffprobe_path = Some(cursor.value(&flag, inline)?),
            "--stage" => {
                request.stages.push(cursor.value(&flag, inline)?);
                request.stage_geometry.push(None);
            }
            "--geometry" => {
                let v = cursor.value(&flag, inline)?;
                match request.stage_geometry.last_mut() {
                    Some(slot) => *slot = Some(v),
                    None => return Err(Error::usage("--geometry describes the --stage given just before it")),
                }
            }
            "--describe" => request.describe = true,
            "--contract" => {
                let v = cursor.value(&flag, inline)?;
                request.contract = Contract::parse(&v).ok_or_else(|| bad_value("--contract", &format!("invalid choice: '{}' (choose from frames/1, frames/2)", v)))?;
            }
            "--stage-timeout" => {
                let v = cursor.value(&flag, inline)?;
                match v.trim().parse::<f64>() {
                    Ok(t) if t.is_finite() && t >= 0.0 => request.stage_timeout = Some(t),
                    _ => return Err(bad_value("--stage-timeout", "expected a number of seconds, 0 or more")),
                }
            }
            other => return Err(Error::usage(format!("unrecognized arguments: {}", other))),
        }
    }
    match positionals.len() {
        0 => return Err(Error::usage("the following arguments are required: input")),
        1 => {}
        2 => request.effect = Some(positionals[1].clone()),
        _ => return Err(Error::usage(format!("unrecognized arguments: {}", positionals[2..].join(" ")))),
    }
    request.input = positionals[0].clone();
    if request.contract == Contract::V1 && (request.describe || request.stage_geometry.iter().any(|g| g.is_some())) {
        let what = if request.describe { "--describe" } else { "--geometry" };
        return Err(Error::usage(format!("{} needs --contract frames/2", what)));
    }
    Ok(Parsed::Run(Box::new(request)))
}

/// Print an error the way the mode asks. `doc_on_stdout` is true for the wrapper, false for
/// `stage apply`, whose standard output carries frames.
fn report(error: &Error, contract: Contract, json: bool, doc_on_stdout: bool, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    if json {
        let doc = error.document(contract, if doc_on_stdout { None } else { Some(error.exit_code()) }).dumps();
        if doc_on_stdout {
            let _ = writeln!(out, "{}", doc);
            let _ = writeln!(err, "{}", error.text_line());
        } else {
            let _ = writeln!(err, "{}", doc);
        }
    } else {
        let _ = writeln!(err, "{}", error.text_line());
    }
    let _ = out.flush();
    contract.exit_for(error)
}

/// Run the command line `args` (without the program name). Returns the process exit code.
pub fn run_cli(args: &[String], stdin: &mut dyn Read, out: &mut dyn Write, err: &mut dyn Write, terminals: Terminals) -> i32 {
    if args.first().map(|a| a.as_str()) == Some("stage") {
        return run_stage_cli(&args[1..], stdin, out, err, terminals);
    }
    let wants_json = args.iter().any(|a| a == "--json");
    let contract = Contract::scan(args);
    let request = match parse_wrapper(args) {
        Ok(Parsed::Help) => {
            let _ = out.write_all(HELP.as_bytes());
            return EXIT_OK;
        }
        Ok(Parsed::Version) => {
            let _ = writeln!(out, "sapphire {}", env!("CARGO_PKG_VERSION"));
            return EXIT_OK;
        }
        Ok(Parsed::Run(r)) => *r,
        Err(e) => return report(&e, contract, wants_json, true, out, err),
    };
    let json = request.json;
    let result = (|| -> Result<()> {
        let prepared = prepare(&request)?;
        if request.describe {
            // the geometry after every declared stage, as a describe answer named after the effect
            let o = prepared.out_spec();
            let answer = crate::geometry::Answer {
                stage: request.effect.clone().unwrap_or_else(|| "stages".into()),
                width: o.width,
                height: o.height,
                depth: o.depth,
                fps: prepared.rate,
                frames: prepared.output_frames(),
            };
            writeln!(out, "{}", answer.to_json().dumps()).map_err(|e| crate::run::io_error("cannot write the answer", &e))?;
            return Ok(());
        }
        if request.dry_run {
            let assembled = dry_run(&prepared);
            if json {
                let doc = prepared.result(&assembled, Vec::new(), None, None);
                writeln!(out, "{}", doc.dumps_indent(2)).map_err(|e| crate::run::io_error("cannot write the result", &e))?;
            } else {
                emit_text(&prepared, &assembled, None, out).map_err(|e| crate::run::io_error("cannot write the plan", &e))?;
            }
            return Ok(());
        }
        let done = execute(&prepared, Vec::new())?;
        for line in crate::run::warnings(&done) {
            let _ = writeln!(err, "warning: {}", line);
        }
        if json {
            writeln!(out, "{}", result_json(&prepared, &done).dumps_indent(2)).map_err(|e| crate::run::io_error("cannot write the result", &e))?;
        } else {
            emit_text(&prepared, &done.assembled, Some(&done), out).map_err(|e| crate::run::io_error("cannot write the result", &e))?;
        }
        Ok(())
    })();
    match result {
        Ok(()) => {
            let _ = out.flush();
            EXIT_OK
        }
        Err(e) => report(&e, request.contract, json, true, out, err),
    }
}

// ----------------------------------------------------------------------------------------------
// sapphire stage ...

struct StageArgs {
    stage: String,
    width: Option<String>,
    height: Option<String>,
    depth: Option<String>,
    fps: Option<String>,
    frames: Option<String>,
    params: Vec<String>,
    layers: Vec<String>,
    input: Option<String>,
    output: Option<String>,
    ffmpeg_path: Option<String>,
    check: bool,
    describe: bool,
    json: bool,
    contract: Contract,
}

fn positive_int(flag: &str, text: &str) -> Result<u32> {
    match text.trim().parse::<u32>() {
        Ok(n) if n >= 1 => Ok(n),
        _ => Err(Error::usage(format!("`{}` expects a positive integer, got `{}`", flag, text))),
    }
}

fn contract_value(text: &str) -> Result<Contract> {
    Contract::parse(text).ok_or_else(|| Error::usage(format!("`--contract` expects frames/1 or frames/2, got `{}`", text)))
}

fn parse_stage_apply(args: &[String]) -> Result<Option<StageArgs>> {
    let mut a = StageArgs {
        stage: String::new(),
        width: None,
        height: None,
        depth: None,
        fps: None,
        frames: None,
        params: Vec::new(),
        layers: Vec::new(),
        input: None,
        output: None,
        ffmpeg_path: None,
        check: false,
        describe: false,
        json: false,
        contract: Contract::default(),
    };
    let mut positionals: Vec<String> = Vec::new();
    let mut cursor = Cursor::new(args);
    while let Some(token) = cursor.next_token() {
        let (flag, inline) = match token {
            Token::Positional(p) => {
                positionals.push(p);
                continue;
            }
            Token::Flag(f) => (f, None),
            Token::FlagValue(f, v) => (f, Some(v)),
        };
        let value = |c: &mut Cursor| c.value(&flag, inline.clone());
        match flag.as_str() {
            "-h" | "--help" => return Ok(None),
            "--width" => a.width = Some(value(&mut cursor)?),
            "--height" => a.height = Some(value(&mut cursor)?),
            "--depth" => a.depth = Some(value(&mut cursor)?),
            "--fps" => a.fps = Some(value(&mut cursor)?),
            "--frames" => a.frames = Some(value(&mut cursor)?),
            "--param" => a.params.push(value(&mut cursor)?),
            "--layer" => a.layers.push(value(&mut cursor)?),
            "--in" => a.input = Some(value(&mut cursor)?),
            "--out" => a.output = Some(value(&mut cursor)?),
            "--ffmpeg-path" => a.ffmpeg_path = Some(value(&mut cursor)?),
            "--check" => a.check = true,
            "--describe" => a.describe = true,
            "--json" => a.json = true,
            "--contract" => a.contract = contract_value(&value(&mut cursor)?)?,
            other => return Err(Error::usage(format!("unknown option `{}`", other))),
        }
    }
    match positionals.len() {
        0 => return Err(Error::usage("missing the stage id, `sapphire stage list` shows the registered ones")),
        1 => a.stage = positionals.remove(0),
        _ => return Err(Error::usage(format!("unexpected argument `{}`", positionals[1]))),
    }
    Ok(Some(a))
}

fn run_stage_cli(args: &[String], stdin: &mut dyn Read, out: &mut dyn Write, err: &mut dyn Write, terminals: Terminals) -> i32 {
    let wants_json = args.iter().any(|a| a == "--json");
    let contract = Contract::scan(args);
    match args.first().map(|s| s.as_str()) {
        None | Some("-h") | Some("--help") | Some("help") => {
            let _ = out.write_all(STAGE_HELP.as_bytes());
            EXIT_OK
        }
        Some("list") => {
            let json = args[1..].iter().any(|a| a == "--json");
            let mut rest = args[1..].iter();
            let mut listed = Contract::default();
            while let Some(a) = rest.next() {
                let checked = match a.as_str() {
                    "--json" | "--all" => Ok(()),
                    "--contract" => rest.next().map(|v| contract_value(v)).unwrap_or_else(|| Err(Error::usage("`--contract` expects a value"))).map(|c| listed = c),
                    other => match other.strip_prefix("--contract=") {
                        Some(v) => contract_value(v).map(|c| listed = c),
                        None => Err(Error::usage(format!("unknown option `{}`", other))),
                    },
                };
                if let Err(e) = checked {
                    return report(&e, contract, json, true, out, err);
                }
            }
            if json {
                let _ = writeln!(out, "{}", stage::list_manifest(listed).dumps_indent(2));
            } else {
                let _ = out.write_all(stage::list_text().as_bytes());
            }
            EXIT_OK
        }
        // a stage process exits with the stage exits (0, 2 usage, 3 unsupported, 4 bad input,
        // other failures) under both contracts; the frames/1 exits 0, 1, 2 belong to wrappers
        Some("apply") => match stage_apply(&args[1..], stdin, out, terminals) {
            Ok(()) => EXIT_OK,
            Err(e) => {
                report(&e, contract, wants_json, false, out, err);
                e.exit_code()
            }
        },
        Some(other) => report(&Error::usage(format!("unknown stage command `{}`: use `list` or `apply`", other)), contract, wants_json, true, out, err),
    }
}

fn stage_apply(args: &[String], stdin: &mut dyn Read, out: &mut dyn Write, terminals: Terminals) -> Result<()> {
    let a = match parse_stage_apply(args)? {
        None => {
            let _ = out.write_all(STAGE_HELP.as_bytes());
            return Ok(());
        }
        Some(a) => a,
    };
    let def = stage::find(&a.stage)
        .ok_or_else(|| Error::unsupported(format!("unknown stage `{}`, `sapphire stage list` shows the registered ones", a.stage)))?;
    if !a.layers.is_empty() {
        return Err(Error::usage(format!("{} is a frame effect, --layer only apply to temporal effects", def.id)));
    }
    let width = positive_int("--width", a.width.as_deref().ok_or_else(|| Error::usage("missing --width"))?)?;
    let height = positive_int("--height", a.height.as_deref().ok_or_else(|| Error::usage("missing --height"))?)?;
    let depth_text = a.depth.as_deref().ok_or_else(|| Error::usage("missing --depth"))?;
    let depth = depth_text
        .trim()
        .parse::<u32>()
        .ok()
        .and_then(Depth::from_bits)
        .ok_or_else(|| Error::usage(format!("`--depth` expects 8, 16 or 32, got `{}`", depth_text)))?;
    let rate = match &a.fps {
        Some(fps) => Some(Rate::parse(fps).ok_or_else(|| Error::usage(format!("`--fps` expects a positive rate such as 24, 29.97 or 30000/1001, got `{}`", fps)))?),
        None => None,
    };
    let frames = match &a.frames {
        None => None,
        Some(text) => Some(positive_int("--frames", text)? as u64),
    };
    let spec = FrameSpec { width, height, depth };
    if spec.frame_bytes() > stage::MAX_FRAME_BYTES {
        return Err(Error::usage(format!("a {}x{} frame at depth {} is more than 4 GiB", width, height, depth.bits())));
    }
    if a.describe && a.contract == Contract::V1 {
        return Err(Error::usage("--describe is part of frames/2; drop --contract frames/1"));
    }
    let values = stage::check_params(&def, &a.params)?;
    let ffmpeg_filter = def.id == crate::ffstage::ID;
    let mut frame_stage = None;
    let mut ff = None;
    if ffmpeg_filter {
        let opts = crate::ffstage::Options::from_values(&values)?;
        let ffmpeg = match a.ffmpeg_path.as_deref() {
            Some(p) => std::path::PathBuf::from(p),
            None => neutral_ffmpeg::Binaries::new().ffmpeg_path().map_err(|e| Error::tool_missing(e.to_string()))?,
        };
        ff = Some((opts, ffmpeg, rate.unwrap_or_else(|| Rate::parse("25/1").expect("a rate"))));
    } else {
        frame_stage = Some(stage::build(&a.stage, spec, &a.params)?);
    }
    if a.describe {
        // the stage reads no input: it says what it would write
        let answer = match &ff {
            Some((opts, ffmpeg, rate)) => crate::ffstage::describe(spec, rate, frames, opts, ffmpeg)?,
            None => crate::geometry::Answer {
                stage: def.id.to_string(),
                width,
                height,
                depth,
                fps: rate.unwrap_or_else(|| Rate::parse("25/1").expect("a rate")),
                frames,
            },
        };
        writeln!(out, "{}", answer.to_json().dumps()).map_err(|e| crate::run::io_error("cannot write the answer", &e))?;
        return Ok(());
    }
    if a.check {
        return Ok(());
    }

    let mut file_in;
    let input: &mut dyn Read = match a.input.as_deref() {
        Some(path) if path != "-" => {
            file_in = std::fs::File::open(path).map_err(|e| Error::bad_input(format!("cannot open input {}: {}", path, e)))?;
            &mut file_in
        }
        _ => {
            if terminals.stdin {
                return Err(Error::usage("refusing to read raw frames from a terminal, pipe them in or use --in"));
            }
            stdin
        }
    };
    let mut file_out;
    let output: &mut dyn Write = match a.output.as_deref() {
        Some(path) if path != "-" => {
            if let Some(input_path) = a.input.as_deref() {
                if crate::paths::same_file(path, input_path) {
                    return Err(Error::usage(format!("--out and {} are the same file", input_path)));
                }
            }
            file_out = std::fs::File::create(path).map_err(|e| Error::bad_input(format!("cannot create output {}: {}", path, e)))?;
            &mut file_out
        }
        _ => {
            if terminals.stdout {
                return Err(Error::usage("refusing to write raw frames to a terminal, pipe them out or use --out"));
            }
            out
        }
    };
    if let Some((opts, ffmpeg, rate)) = &ff {
        crate::ffstage::apply(spec, rate, frames, opts, ffmpeg, &mut ReadRef(input), &mut WriteRef(output))?;
        return Ok(());
    }
    let mut frame_stage = frame_stage.expect("a frame stage");
    stage::run_frames(frame_stage.as_mut(), spec, &mut ReadRef(input), &mut WriteRef(output), frames)?;
    Ok(())
}

/// Adapters from `&mut dyn Read` / `&mut dyn Write` to the generic functions of [`stage`].
struct ReadRef<'a>(&'a mut dyn Read);
struct WriteRef<'a>(&'a mut dyn Write);

impl Read for ReadRef<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for WriteRef<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Code;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn run(list: &[&str], input: &[u8]) -> (i32, Vec<u8>, String) {
        let mut stdin = input;
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = run_cli(&args(list), &mut stdin, &mut out, &mut err, Terminals::default());
        (code, out, String::from_utf8_lossy(&err).into_owned())
    }

    #[test]
    fn the_wrapper_reads_the_flags_of_apply_py() {
        let parsed = parse_wrapper(&args(&[
            "clip.mp4", "retime", "--param", "speed_percent=50", "--param=x=1", "--layer", "alt=b.mp4", "--frames", "90", "--depth", "32", "--vfx", "v.exe", "-o", "out.mp4",
            "--dry-run", "--json", "--force", "--ffmpeg-path", "ff", "--ffprobe-path", "fp", "--stage", "a b", "--stage-timeout", "2.5",
        ]))
        .ok()
        .unwrap();
        let r = match parsed {
            Parsed::Run(r) => *r,
            _ => panic!("not a run"),
        };
        assert_eq!((r.input.as_str(), r.effect.as_deref()), ("clip.mp4", Some("retime")));
        assert_eq!(r.params, ["speed_percent=50", "x=1"]);
        assert_eq!(r.layers, [("alt".to_string(), "b.mp4".to_string())]);
        assert_eq!((r.frames, r.depth, r.vfx.as_deref(), r.output.as_deref()), (Some(90), Some(Depth::F32), Some("v.exe"), Some("out.mp4")));
        assert!(r.dry_run && r.json && r.force && !r.lossless);
        assert_eq!((r.ffmpeg_path.as_deref(), r.ffprobe_path.as_deref()), (Some("ff"), Some("fp")));
        assert_eq!(r.stages, ["a b"]);
        assert_eq!(r.stage_timeout, Some(2.5));
    }

    fn parse_err(list: &[&str]) -> String {
        match parse_wrapper(&args(list)) {
            Err(e) => {
                assert_eq!(e.code, Code::Usage, "{:?}", list);
                e.message
            }
            Ok(_) => panic!("{:?} should not parse", list),
        }
    }

    #[test]
    fn wrapper_mistakes_are_usage_errors_in_argparse_words() {
        assert_eq!(parse_err(&["a.mp4", "retime", "--param", "novalue"]), "argument --param: expected K=V, for example speed_percent=40");
        assert_eq!(parse_err(&["a.mp4", "retime", "--param", "=1"]), "argument --param: expected K=V, for example speed_percent=40");
        assert_eq!(parse_err(&["a.mp4", "retime", "--layer", "x"]), "argument --layer: expected ID=VIDEO, for example overlay=overlay.mp4");
        assert_eq!(parse_err(&["a.mp4", "retime", "--frames", "0"]), "argument --frames: expected a positive number of frames");
        assert_eq!(parse_err(&["a.mp4", "retime", "--frames", "x"]), "argument --frames: expected a positive number of frames");
        assert_eq!(parse_err(&["a.mp4", "retime", "--depth", "12"]), "argument --depth: invalid choice: '12' (choose from 8, 16, 32)");
        assert_eq!(parse_err(&["a.mp4", "retime", "--depth", "x"]), "argument --depth: invalid int value: 'x'");
        assert_eq!(parse_err(&["a.mp4", "retime", "--stage-timeout", "-1"]), "argument --stage-timeout: expected a number of seconds, 0 or more");
        assert_eq!(parse_err(&["a.mp4", "retime", "--output"]), "argument --output: expected one argument");
        assert_eq!(parse_err(&["a.mp4", "retime", "--nope"]), "unrecognized arguments: --nope");
        assert_eq!(parse_err(&["a.mp4", "retime", "extra"]), "unrecognized arguments: extra");
        assert_eq!(parse_err(&[]), "the following arguments are required: input");
    }

    #[test]
    fn help_prints_and_exits_zero() {
        let (code, out, err) = run(&["--help"], b"");
        assert_eq!(code, 0);
        assert!(String::from_utf8_lossy(&out).starts_with("usage: sapphire INPUT [EFFECT]"));
        assert!(err.is_empty());
        let (code, out, _) = run(&["stage", "--help"], b"");
        assert_eq!(code, 0);
        assert!(String::from_utf8_lossy(&out).contains("stage apply STAGE"));
    }

    #[test]
    fn a_usage_error_prints_the_text_line_and_exits_2() {
        let (code, out, err) = run(&["a.mp4", "retime", "--nope"], b"");
        assert_eq!(code, 2);
        assert!(out.is_empty());
        assert_eq!(err.trim(), "error: usage: unrecognized arguments: --nope");
    }

    #[test]
    fn json_mode_prints_the_error_object_on_stdout() {
        let (code, out, err) = run(&["a.mp4", "retime", "--nope", "--json"], b"");
        assert_eq!(code, 2);
        let doc = String::from_utf8_lossy(&out).into_owned();
        assert!(doc.starts_with(r#"{"contract": "frames/2", "ok": false, "error": {"code": "usage", "stage": "args", "exit_code": null, "message": "unrecognized arguments: --nope""#), "{}", doc);
        assert!(err.starts_with("error: usage:"));
    }

    #[test]
    fn stage_list_json_carries_the_contract() {
        let (code, out, _) = run(&["stage", "list", "--json"], b"");
        assert_eq!(code, 0);
        let text = String::from_utf8_lossy(&out).into_owned();
        assert!(text.contains("\"contract\": \"frames/2\""));
        assert!(text.contains("selftest-invert"));
        let (code, out, _) = run(&["stage", "list"], b"");
        assert_eq!(code, 0);
        assert!(String::from_utf8_lossy(&out).contains("selftest-invert"));
        let (code, _, err) = run(&["stage", "list", "--bogus"], b"");
        assert_eq!(code, 2);
        assert!(err.contains("unknown option `--bogus`"));
    }

    #[test]
    fn stage_apply_inverts_a_stream() {
        let frame = vec![10u8, 20, 30, 40, 50, 60, 70, 80];
        let (code, out, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--fps", "24"], &frame);
        assert_eq!((code, err.as_str()), (0, ""));
        assert_eq!(out, vec![245, 235, 225, 40, 205, 195, 185, 80]);
        let (code, again, _) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8"], &out);
        assert_eq!(code, 0);
        assert_eq!(again, frame);
    }

    #[test]
    fn stage_apply_follows_the_vfx_exit_codes() {
        let base = ["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8"];
        let mut v = base.to_vec();
        v.extend(["--param", "nope=1"]);
        let (code, _, err) = run(&v, b"");
        assert_eq!((code, err.trim()), (3, "error: unsupported: unknown param `nope`"));

        let (code, out, err) = run(&base, &[1u8; 8 + 3]);
        assert_eq!(code, 4);
        assert_eq!(out.len(), 8, "the whole frame before the partial one stays written");
        assert_eq!(err.trim(), "error: partial_frame: frame 1: input ends with a partial frame (3 of 8 bytes) after 1 whole frames");

        let (code, _, err) = run(&["stage", "apply", "nope", "--width", "2", "--height", "1", "--depth", "8"], b"");
        assert_eq!(code, 3);
        assert!(err.starts_with("error: unsupported: unknown stage `nope`"));

        let (code, _, err) = run(&["stage", "apply", "selftest-invert", "--height", "1", "--depth", "8"], b"");
        assert_eq!((code, err.trim()), (2, "error: usage: missing --width"));

        let (code, _, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "12"], b"");
        assert_eq!((code, err.trim()), (2, "error: usage: `--depth` expects 8, 16 or 32, got `12`"));

        let (code, _, err) = run(&["stage", "apply", "selftest-invert", "--width", "0", "--height", "1", "--depth", "8"], b"");
        assert_eq!((code, err.trim()), (2, "error: usage: `--width` expects a positive integer, got `0`"));

        let (code, _, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--fps", "0"], b"");
        assert_eq!(code, 2);
        assert!(err.contains("`--fps` expects a positive rate"));

        let mut layered = base.to_vec();
        layered.extend(["--layer", "a=b"]);
        let (code, _, err) = run(&layered, b"");
        assert_eq!((code, err.trim()), (2, "error: usage: selftest-invert is a frame effect, --layer only apply to temporal effects"));
    }

    #[test]
    fn stage_apply_check_reads_nothing() {
        let (code, out, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--check"], b"junk");
        assert_eq!((code, out.len(), err.as_str()), (0, 0, ""));
        let (code, _, err) = run(
            &["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--check", "--param", "invert_alpha=maybe"],
            b"",
        );
        assert_eq!(code, 3);
        assert!(err.contains("bad value for `invert_alpha`"));
    }

    #[test]
    fn stage_apply_checks_the_frame_count() {
        let frames = vec![0u8; 16];
        let (code, out, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--frames", "3"], &frames);
        assert_eq!(code, 4);
        assert_eq!(out.len(), 16);
        assert_eq!(err.trim(), "error: bad_input: the input holds 2 frames, but --frames says 3");
    }

    #[test]
    fn stage_apply_json_errors_go_to_stderr_as_one_line() {
        let (code, out, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--json"], &[1u8; 5]);
        assert_eq!(code, 4);
        assert!(out.is_empty(), "stdout carries frames only");
        assert!(err.trim().starts_with(r#"{"contract": "frames/2", "ok": false, "error": {"code": "partial_frame", "stage": null, "exit_code": null"#), "{}", err);
        assert_eq!(err.trim().lines().count(), 1);
        let (code, _, err) = run(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8", "--json", "--contract", "frames/1"], &[1u8; 5]);
        assert_eq!(code, 4, "a stage keeps the stage exits under frames/1");
        assert!(err.trim().starts_with(r#"{"contract": "frames/1", "ok": false, "error": {"code": "partial_frame", "stage": null, "exit_code": 4"#), "{}", err);
    }

    #[test]
    fn a_terminal_is_refused() {
        let mut stdin: &[u8] = b"";
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = run_cli(
            &args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "1", "--depth", "8"]),
            &mut stdin,
            &mut out,
            &mut err,
            Terminals { stdin: true, stdout: false },
        );
        assert_eq!(code, 2);
        assert!(String::from_utf8_lossy(&err).contains("refusing to read raw frames from a terminal"));
    }

    #[test]
    fn the_stage_command_needs_a_known_verb() {
        let (code, _, err) = run(&["stage", "frobnicate"], b"");
        assert_eq!(code, 2);
        assert!(err.contains("unknown stage command `frobnicate`"));
        let (code, _, err) = run(&["stage", "apply"], b"");
        assert_eq!(code, 2);
        assert!(err.contains("missing the stage id"));
    }
}
