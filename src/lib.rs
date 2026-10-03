//! biomarker-cli: track biomarkers for any number of people in a local
//! FrankenSQLite (fsqlite) database.

#![recursion_limit = "512"]

pub mod cli;
pub mod commands;
pub mod config;
pub mod context;
pub mod db;
pub mod error;
pub mod migrations;
pub mod output;
pub mod ranges;
pub mod seed;
pub mod stats;
pub mod store;
pub mod units;
pub mod util;
pub mod view;

use clap::Parser;

use crate::error::ErrorKind;

/// Parse arguments, run the command and return the process exit code.
pub fn run<I, T>(args: I) -> i32
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = match cli::Cli::try_parse_from(args) {
        Ok(c) => c,
        Err(e) => {
            let _ = e.print();
            return if e.use_stderr() { ErrorKind::Usage.exit_code() } else { 0 };
        }
    };
    let is_json = |f: Option<output::Format>| matches!(f, Some(output::Format::Json | output::Format::Jsonl));
    let mut json_errors = is_json(cli.global.format);
    let result = context::Ctx::new(&cli.global).and_then(|ctx| {
        json_errors = is_json(Some(ctx.out.format));
        commands::dispatch(&ctx, cli.command)
    });
    match result {
        Ok(code) => code,
        Err(e) => {
            if json_errors {
                eprintln!(
                    "{}",
                    serde_json::json!({"schema": output::SCHEMA, "kind": "error", "error": {"kind": e.kind.as_str(), "code": e.exit_code(), "message": e.message}})
                );
            } else {
                eprintln!("biomarker: error: {e}");
            }
            e.exit_code()
        }
    }
}
