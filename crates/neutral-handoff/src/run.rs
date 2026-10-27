//! Running a [`Prepared`] plan: the scratch folder, the layer clips, the pipeline, the checks of
//! the finished file, and the move into place.
//!
//! The order is that of `tools/apply.py`:
//!
//! 1. the destination folder is made and a left-over partial file removed;
//! 2. a scratch folder (`apply_XXXXXXXX` in the system temp folder) is made;
//! 3. each `--layer` video is decoded to a raw clip of the effect's format, one after the other;
//! 4. decode, the stages and encode run as one pipeline writing `<output stem>.partial-<pid>.<ext>`;
//! 5. the partial file is probed and verified against the input;
//! 6. it is moved onto the output path.
//!
//! Whatever happens, the partial file and the scratch folder are removed at the end, and the
//! output path is touched only by step 6.

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

use crate::error::{tail_lines, Code, Error, Result};
use crate::media::{probe, Info};
use crate::paths::{discard, make_scratch, temp_dir};
use crate::pipeline::{self, PipelineConfig, RunReport, StageReport};
use crate::plan::{Assembled, Prepared};
use crate::pyjson::Out;
use crate::stage::FrameStage;
use crate::sys::command;
use crate::tools::process_env;
use crate::verify::{verify, verify_with};

/// What a finished run produced.
#[derive(Debug)]
pub struct Executed {
    /// The plan and commands that ran.
    pub assembled: Assembled,
    /// The probe of the output, after it was moved into place.
    pub output: Info,
    /// What the pipeline did.
    pub run: RunReport,
    /// Wall time of the whole run.
    pub seconds: f64,
}

/// The plan for a dry run: the scratch folder is named, not made.
pub fn dry_run(prepared: &Prepared) -> Assembled {
    let scratch = temp_dir(&process_env).join("apply_XXXXXXXX");
    prepared.assemble(&scratch.to_string_lossy())
}

/// Run the plan. `hooks` replaces an in-process stage by its step name (see [`pipeline::run`]).
pub fn execute(prepared: &Prepared, hooks: Vec<(String, Box<dyn FrameStage>)>) -> Result<Executed> {
    let started = Instant::now();
    let dst = prepared.dst.clone();
    if let Some(parent) = Path::new(&std::path::absolute(&dst).unwrap_or_else(|_| dst.clone().into())).parent() {
        fs::create_dir_all(parent).map_err(|e| Error::bad_input(format!("cannot create the folder of {}: {}", dst, e)).at("plan"))?;
    }
    let partial = prepared.partial();
    discard(&partial);
    let scratch = make_scratch(&temp_dir(&process_env))
        .map_err(|e| Error::bad_input(format!("cannot create a scratch folder: {}", e)).at("plan"))?;
    let scratch_text = scratch.to_string_lossy().into_owned();

    let result = (|| -> Result<Executed> {
        let assembled = prepared.assemble(&scratch_text);
        decode_layers(prepared, &assembled, &scratch)?;
        let config = pipeline_config(prepared, &assembled);
        let run = pipeline::run(&config, hooks)?;
        // ffmpeg exits 0 on a truncated or damaged file, so the frames are counted here
        if let Some(stated) = assembled.plan.input_frames {
            if run.decoded < stated {
                return Err(Error::bad_input(format!(
                    "the input decoded to {} frames, but its container states {}: the file may be truncated or damaged",
                    run.decoded, stated
                ))
                .at("decode")
                .with_tail(tail_lines(&run.decode_stderr, 3)));
            }
        }

        let produced = probe(&prepared.media.ffprobe, &partial);
        let problems = if prepared.geometry_changed() {
            let out = prepared.out_spec();
            let frames = prepared.output_frames();
            verify_with(&prepared.info, produced.as_ref(), &prepared.rate, frames, None, Some((out.width, out.height)), frames.is_some())
        } else {
            verify(&prepared.info, produced.as_ref(), &prepared.rate, prepared.request.frames, assembled.plan.input_frames)
        };
        if !problems.is_empty() {
            return Err(Error::stage_failed(format!("the output did not verify: {}", problems.join("; "))).at("verify"));
        }
        if let Err(e) = fs::rename(&partial, &dst) {
            return Err(Error::bad_input(format!(
                "could not move the finished output into place; is the destination open in another program? ({})",
                e
            ))
            .at("verify"));
        }
        let output = probe(&prepared.media.ffprobe, &dst).ok_or_else(|| Error::stage_failed("output verification failed").at("verify"))?;
        Ok(Executed { assembled, output, run, seconds: 0.0 })
    })();

    discard(&partial);
    let _ = fs::remove_dir_all(&scratch);
    result.map(|mut done| {
        done.seconds = (started.elapsed().as_secs_f64() * 100.0).round() / 100.0;
        done
    })
}

/// What the pipeline needs of a plan.
pub fn pipeline_config(prepared: &Prepared, assembled: &Assembled) -> PipelineConfig {
    PipelineConfig {
        media: prepared.media.clone(),
        input: prepared.request.input.clone(),
        video_ordinal: prepared.info.video_ordinal.unwrap_or(0),
        decode_filter: prepared.decode_filter(),
        spec: prepared.spec,
        rate: prepared.rate,
        encode_tail: assembled.commands.encode_tail.clone(),
        partial: prepared.partial(),
        steps: assembled.commands.steps.clone(),
        stage_timeout: match prepared.request.stage_timeout() {
            t if t > 0.0 => Some(Duration::from_secs_f64(t)),
            _ => None,
        },
    }
}

/// Decode each layer video to a raw clip of the effect's format, size and rate.
fn decode_layers(prepared: &Prepared, assembled: &Assembled, scratch: &Path) -> Result<()> {
    let frame_bytes = prepared.spec.frame_bytes();
    for (layer, argv) in assembled.plan.layers.iter().zip(&assembled.commands.layers) {
        let stage = format!("layer:{}", layer.id);
        let log_path = scratch.join(format!("layer_{}.log", layer.id));
        let log = fs::File::create(&log_path).map_err(|e| Error::bad_input(format!("cannot write {}: {}", log_path.display(), e)).at(&stage))?;
        let status = command(&argv[0])
            .args(&argv[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(log)
            .status()
            .map_err(|e| Error::tool_missing(format!("cannot start ffmpeg for layer {}: {}", layer.id, e)).at(&stage))?;
        if !status.success() {
            let text = fs::read_to_string(&log_path).unwrap_or_default();
            return Err(Error::bad_input(format!(
                "layer {}: decoding {} failed (ffmpeg exited with code {})",
                layer.id,
                layer.video,
                status.code().map(|c| c.to_string()).unwrap_or_else(|| "none".into())
            ))
            .at(&stage)
            .with_tail(tail_lines(&text, 3)));
        }
        let size = fs::metadata(&layer.temp).map(|m| m.len()).unwrap_or(0);
        if size == 0 || size % frame_bytes != 0 {
            return Err(Error::bad_input(format!(
                "layer {}: {} decoded to {} bytes, which is not a whole number of {}x{} frames",
                layer.id, layer.video, size, prepared.spec.width, prepared.spec.height
            ))
            .at(&stage));
        }
    }
    Ok(())
}

/// The report of one stage in `--json` form.
pub fn stage_report_json(report: &StageReport) -> Out {
    Out::map(vec![
        ("name", Out::text(report.name.clone())),
        ("command", Out::text(report.command.clone())),
        ("stderr_tail", Out::List(report.stderr_tail.iter().cloned().map(Out::Str).collect())),
        ("exit_code", Out::opt_int(report.exit_code.map(|c| c as i64))),
        ("frames_in", Out::Int(report.frames_in as i64)),
        ("frames_out", Out::Int(report.frames_out as i64)),
        ("seconds", Out::Float((report.seconds * 100.0).round() / 100.0)),
    ])
}

/// The `--json` document of a finished run.
pub fn result_json(prepared: &Prepared, done: &Executed) -> Out {
    let output = Out::map(vec![
        ("path", Out::text(std::path::absolute(&prepared.dst).unwrap_or_else(|_| prepared.dst.clone().into()).to_string_lossy())),
        ("probe", done.output.to_json()),
    ]);
    let reports = done.run.stages.iter().map(stage_report_json).collect();
    let mut doc = prepared.result(&done.assembled, vec![output], Some(done.seconds), Some(reports));
    if let Out::Map(members) = &mut doc {
        if prepared.request.contract == crate::error::Contract::V2 {
            for member in members.iter_mut() {
                if member.0 == "stages" {
                    member.1 = prepared.report_stages(&done.assembled, Some(&done.run));
                }
            }
        }
        members.push(("warnings".into(), Out::List(warnings(done).into_iter().map(Out::Str).collect())));
    }
    doc
}

/// What ffmpeg printed during a run that still succeeded: damaged data it skipped, for example.
pub fn warnings(done: &Executed) -> Vec<String> {
    let mut lines = Vec::new();
    for (who, text) in [("decode", &done.run.decode_stderr), ("encode", &done.run.encode_stderr)] {
        lines.extend(tail_lines(text, 5).into_iter().filter(|l| !l.trim().is_empty()).map(|l| format!("{}: {}", who, l)));
    }
    lines
}

/// Print a plan or a result the way `apply.py` does in text mode.
///
/// `[plan] summary`, `[temp] path (what)`, one `[cmd]` line per command, then
/// `[dry-run] nothing was written`, or the output path with its facts as the last line.
pub fn emit_text(prepared: &Prepared, assembled: &Assembled, done: Option<&Executed>, out: &mut dyn Write) -> std::io::Result<()> {
    writeln!(out, "[plan] {}", prepared.summary(&assembled.plan))?;
    for temp in &assembled.plan.temp_files {
        writeln!(out, "[temp] {} ({})", temp.path, temp.what)?;
    }
    for line in assembled.commands.lines() {
        writeln!(out, "[cmd] {}", line)?;
    }
    match done {
        None => writeln!(out, "[dry-run] nothing was written")?,
        Some(done) => {
            let path = std::path::absolute(&prepared.dst).unwrap_or_else(|_| prepared.dst.clone().into());
            writeln!(out, "{}: {}", path.display(), done.output.describe())?;
        }
    }
    Ok(())
}

/// A code for a failure that has no better place; used by the command line for I/O on its own streams.
pub fn io_error(what: &str, e: &std::io::Error) -> Error {
    Error::new(Code::BadInput, format!("{}: {}", what, e))
}
