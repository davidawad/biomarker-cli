//! Per-invocation context: resolved configuration, output options, database access.

use std::io::IsTerminal;
use std::path::PathBuf;

use crate::cli::GlobalOpts;
use crate::config::{self, Layer, Resolved};
use crate::db::Db;
use crate::error::Result;
use crate::output::{Format, OutputOpts, Report};
use crate::util::Tz;

pub struct Ctx {
    pub resolved: Resolved,
    pub out: OutputOpts,
    pub tz: Tz,
    pub db_path: PathBuf,
}

/// Translate global flags into the highest-precedence config layer.
pub fn flag_layer(g: &GlobalOpts) -> Layer {
    let s = |k: &str, v: &Option<String>| v.as_ref().map(|v| (k.to_string(), v.clone()));
    [
        g.db.as_ref().map(|p| ("db_path".to_string(), p.to_string_lossy().into_owned())),
        g.format.map(|f| ("format".to_string(), format!("{f:?}").to_lowercase())),
        s("unit_system", &g.units),
        s("precision", &g.precision),
        s("date_format", &g.date_format),
        s("timezone", &g.timezone),
        s("color", &g.color),
        s("csv_delimiter", &g.delimiter),
        s("csv_quote", &g.quote),
        g.no_header.then(|| ("csv_header".to_string(), "false".to_string())),
        s("null", &g.null),
        s("range_flavor", &g.range_flavor),
        g.quiet.then(|| ("quiet".to_string(), "true".to_string())),
        g.verbose.then(|| ("verbose".to_string(), "true".to_string())),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn first_byte(s: &str, fallback: u8) -> u8 {
    s.bytes().next().unwrap_or(fallback)
}

impl Ctx {
    pub fn new(g: &GlobalOpts) -> Result<Self> {
        let resolved = config::resolve(g.config.as_deref(), flag_layer(g))?;
        let to_file = g.output.as_ref().is_some_and(|p| p.as_os_str() != "-");
        let color = match resolved.get("color") {
            "always" => true,
            "never" => false,
            _ => !to_file && std::io::stdout().is_terminal(),
        };
        let out = OutputOpts {
            format: Format::parse(resolved.get("format"))?,
            output: g.output.clone(),
            precision: resolved.get("precision").parse().unwrap_or(2),
            delimiter: first_byte(resolved.get("csv_delimiter"), b','),
            quote: first_byte(resolved.get("csv_quote"), b'"'),
            header: resolved.flag("csv_header"),
            null: resolved.get("null").to_string(),
            color,
            date_format: resolved.get("date_format").to_string(),
            columns: g.columns.clone(),
        };
        let tz = Tz::parse(resolved.get("timezone"))?;
        let db_path = PathBuf::from(expand_tilde(resolved.get("db_path")));
        Ok(Self { resolved, out, tz, db_path })
    }

    pub fn db(&self) -> Result<Db> {
        self.verbose(&format!("database: {}", self.db_path.display()));
        Db::open(&self.db_path)
    }

    pub fn quiet(&self) -> bool {
        self.resolved.flag("quiet")
    }

    /// Informational message on stderr (suppressed by --quiet).
    pub fn info(&self, msg: &str) {
        if !self.quiet() {
            eprintln!("{msg}");
        }
    }

    pub fn verbose(&self, msg: &str) {
        if self.resolved.flag("verbose") {
            eprintln!("biomarker: {msg}");
        }
    }

    pub fn emit(&self, r: &Report) -> Result<()> {
        crate::output::emit(r, &self.out)
    }

    /// Emit the result of a mutating command. In table mode the stderr status
    /// line is enough for humans, so nothing is printed to stdout.
    pub fn emit_mutation(&self, r: &Report) -> Result<()> {
        if self.out.format == Format::Table && self.out.output.is_none() {
            Ok(())
        } else {
            self.emit(r)
        }
    }

    pub fn default_person(&self) -> Option<String> {
        Some(self.resolved.get("default_person").to_string()).filter(|s| !s.is_empty())
    }

    pub fn unit_system(&self) -> &str {
        self.resolved.get("unit_system")
    }

    pub fn range_flavor(&self) -> &str {
        self.resolved.get("range_flavor")
    }

    /// Extra date format accepted when parsing input dates.
    pub fn input_date_formats(&self) -> Vec<String> {
        vec![self.out.date_format.clone()]
    }
}

pub fn expand_tilde(p: &str) -> String {
    match (p.strip_prefix("~/"), std::env::var("HOME")) {
        (Some(rest), Ok(home)) => format!("{home}/{rest}"),
        _ => p.to_string(),
    }
}
