//! The `sapphire` binary: the pipeline wrapper and the `stage` subcommand.
//!
//! See [`neutral_handoff::cli`] for the command line and [`neutral_handoff::error`] for the
//! exit codes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io::{IsTerminal, Write};

use neutral_handoff::cli::{run_cli, Terminals};

fn main() {
    let args: Vec<String> = std::env::args_os().skip(1).map(|a| a.to_string_lossy().into_owned()).collect();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let terminals = Terminals { stdin: stdin.is_terminal(), stdout: stdout.is_terminal() };
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut errors = stderr.lock();
    let code = run_cli(&args, &mut input, &mut output, &mut errors, terminals);
    let _ = output.flush();
    let _ = errors.flush();
    drop(output);
    std::process::exit(code);
}
