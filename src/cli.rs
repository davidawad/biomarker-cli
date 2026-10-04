//! Command-line interface definition (clap derive).

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::output::Format;

#[derive(Debug, Parser)]
#[command(
    name = "biomarker",
    version,
    about = "Track biomarkers (lab results) for any number of people",
    long_about = "Track biomarkers (lab results) for any number of people.\n\n\
        Data lives in a local FrankenSQLite (fsqlite) database. Every command can \
        emit table, json, jsonl, csv or tsv output; JSON output uses the versioned \
        `biomarker/v1` envelope documented in docs/json-schema.md.",
    after_help = "Exit codes: 0 ok, 1 error, 2 usage, 3 not found, 4 invalid data, 5 database, 6 io, 7 config, 8 key (database locked: missing or wrong encryption key), 10 flagged values found (flag --exit-code)."
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Args, Default)]
pub struct GlobalOpts {
    /// Config file (default: biomarker-cli/config.toml under $XDG_CONFIG_HOME, ~/.config,
    /// or %APPDATA% on Windows; env BIOMARKER_CONFIG)
    #[arg(long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,
    /// Database file (env BIOMARKER_DB)
    #[arg(long, global = true, value_name = "FILE")]
    pub db: Option<PathBuf>,
    /// Output format
    #[arg(short = 'f', long, global = true, value_enum)]
    pub format: Option<Format>,
    /// Write output to FILE instead of stdout
    #[arg(short = 'o', long, global = true, value_name = "FILE")]
    pub output: Option<PathBuf>,
    /// Display units: canonical, us or si
    #[arg(long, global = true, value_name = "SYSTEM")]
    pub units: Option<String>,
    /// Decimal places for table/csv/tsv output
    #[arg(long, global = true, value_name = "N")]
    pub precision: Option<String>,
    /// strftime date format for table/csv/tsv output (also accepted on import)
    #[arg(long, global = true, value_name = "FMT")]
    pub date_format: Option<String>,
    /// Time zone: local, UTC, IANA name or +HH:MM
    #[arg(long = "tz", global = true, value_name = "TZ")]
    pub timezone: Option<String>,
    /// Colour: auto, always, never
    #[arg(long, global = true, value_name = "WHEN")]
    pub color: Option<String>,
    /// CSV delimiter character (or 'tab')
    #[arg(long, global = true, value_name = "CHAR")]
    pub delimiter: Option<String>,
    /// CSV quote character
    #[arg(long, global = true, value_name = "CHAR")]
    pub quote: Option<String>,
    /// Omit the header row in csv/tsv output
    #[arg(long, global = true)]
    pub no_header: bool,
    /// Text for missing values in table/csv/tsv output
    #[arg(long, global = true, value_name = "TEXT")]
    pub null: Option<String>,
    /// Ranges used for flagging: reference, optimal, both
    #[arg(long, global = true, value_name = "FLAVOR")]
    pub range_flavor: Option<String>,
    /// Comma-separated list of columns to output (table/csv/tsv)
    #[arg(long, global = true, value_name = "COLS", value_delimiter = ',')]
    pub columns: Option<Vec<String>>,
    /// Suppress informational messages
    #[arg(short, long, global = true)]
    pub quiet: bool,
    /// Print extra diagnostics to stderr
    #[arg(short, long, global = true)]
    pub verbose: bool,
    /// Allow an UNENCRYPTED database (create one, or open a legacy plaintext one)
    #[arg(long, global = true)]
    pub insecure_plaintext: bool,
    /// Encrypt --output (or stdout) as an ASCII-armored age file
    #[arg(long, global = true)]
    pub encrypt_output: bool,
    /// age X25519 recipient (age1...) for --encrypt-output (repeatable; default: passphrase)
    #[arg(long = "recipient", global = true, value_name = "AGE_PUBKEY")]
    pub recipients: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage people
    #[command(subcommand)]
    Person(PersonCmd),
    /// Manage the marker catalog
    #[command(subcommand)]
    Marker(MarkerCmd),
    /// Manage reference and optimal ranges
    #[command(subcommand)]
    Range(RangeCmd),
    /// Units and conversions
    #[command(subcommand)]
    Unit(UnitCmd),
    /// Record a single measurement
    Add(AddArgs),
    /// Remove a measurement by id
    Rm(RmMeasurementArgs),
    /// Import measurements from CSV, JSON, JSONL or a spreadsheet (xlsx/xls/ods)
    Import(ImportArgs),
    /// Export measurements (CSV/JSON/JSONL, re-importable)
    Export(ExportArgs),
    /// Query measurements
    #[command(alias = "list", alias = "ls")]
    Query(QueryArgs),
    /// Latest value per person and marker
    Latest(QueryArgs),
    /// Trend statistics (min/max/mean/median/slope, % change over windows)
    #[command(alias = "stats")]
    Trend(TrendArgs),
    /// Show values outside reference/optimal ranges
    Flag(FlagArgs),
    /// Qualitative results (e.g. "Negative", "1+ Abnormal") kept beside the numbers
    #[command(alias = "obs")]
    Observations(ObservationsArgs),
    /// Compare values between two dates
    Diff(DiffArgs),
    /// Database maintenance
    #[command(subcommand)]
    Db(DbCmd),
    /// Configuration
    #[command(subcommand)]
    Config(ConfigCmd),
    /// Encrypted audit trail of commands that read or modify data
    #[command(subcommand)]
    Audit(AuditCmd),
    /// Check encryption, keys, file permissions and the audit log
    Doctor,
    /// Generate shell completions
    Completions(CompletionsArgs),
    /// Generate man page(s)
    Man(ManArgs),
}

// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Sex {
    Male,
    Female,
    Other,
}

impl Sex {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum PersonCmd {
    /// Add a person
    Add(PersonAddArgs),
    /// List people
    #[command(alias = "ls")]
    List,
    /// Show a person with measurement summary
    Show { slug: String },
    /// Edit a person
    Edit(PersonEditArgs),
    /// Remove a person (and, with --force, their measurements)
    Rm {
        slug: String,
        /// Delete even if the person has measurements
        #[arg(long)]
        force: bool,
    },
}

#[derive(Debug, Args)]
pub struct PersonAddArgs {
    /// Short identifier, e.g. `alice`
    pub slug: String,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long, value_enum)]
    pub sex: Option<Sex>,
    /// Date of birth (YYYY-MM-DD)
    #[arg(long)]
    pub dob: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
    /// Tag (repeatable or comma-separated)
    #[arg(long = "tag", value_delimiter = ',')]
    pub tags: Vec<String>,
}

#[derive(Debug, Args)]
pub struct PersonEditArgs {
    pub slug: String,
    /// New slug
    #[arg(long)]
    pub rename: Option<String>,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long, value_enum)]
    pub sex: Option<Sex>,
    #[arg(long)]
    pub dob: Option<String>,
    #[arg(long)]
    pub notes: Option<String>,
    #[arg(long = "add-tag", value_delimiter = ',')]
    pub add_tags: Vec<String>,
    #[arg(long = "rm-tag", value_delimiter = ',')]
    pub rm_tags: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum MarkerCmd {
    /// Add a marker to the catalog
    Add(MarkerAddArgs),
    /// List markers
    #[command(alias = "ls")]
    List {
        /// Filter by category
        #[arg(long)]
        category: Option<String>,
        /// Case-insensitive substring search over slug, name and aliases
        #[arg(long)]
        search: Option<String>,
    },
    /// Show a marker with aliases, ranges and conversions
    Show { marker: String },
    /// Edit a marker
    Edit(MarkerEditArgs),
    /// Remove a marker (and, with --force, its measurements)
    Rm {
        marker: String,
        #[arg(long)]
        force: bool,
    },
    /// Add (or with --remove, delete) aliases
    Alias {
        marker: String,
        aliases: Vec<String>,
        #[arg(long)]
        remove: bool,
    },
    /// List marker categories
    Categories,
}

#[derive(Debug, Args)]
pub struct MarkerAddArgs {
    pub slug: String,
    #[arg(long)]
    pub name: Option<String>,
    /// Canonical unit
    #[arg(long)]
    pub unit: String,
    #[arg(long, default_value = "other")]
    pub category: String,
    #[arg(long)]
    pub loinc: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long = "alias", value_delimiter = ',')]
    pub aliases: Vec<String>,
}

#[derive(Debug, Args)]
pub struct MarkerEditArgs {
    pub marker: String,
    #[arg(long)]
    pub rename: Option<String>,
    #[arg(long)]
    pub name: Option<String>,
    /// New canonical unit (stored values and ranges are converted)
    #[arg(long)]
    pub unit: Option<String>,
    #[arg(long)]
    pub category: Option<String>,
    #[arg(long)]
    pub loinc: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum KindArg {
    Reference,
    Optimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RangeSex {
    Any,
    Male,
    Female,
}

#[derive(Debug, Subcommand)]
pub enum RangeCmd {
    /// Set (insert or replace) a range
    Set(RangeSetArgs),
    /// List ranges
    #[command(alias = "ls")]
    List {
        /// Only ranges of this marker
        marker: Option<String>,
        #[arg(long, value_enum)]
        kind: Option<KindArg>,
        /// Only ranges for this person (their own ranges plus the catalog's)
        #[arg(short, long)]
        person: Option<String>,
    },
    /// Remove a range by id
    Rm {
        id: i64,
        /// The id is a person-specific range (see `range list --person`)
        #[arg(long)]
        personal: bool,
    },
}

#[derive(Debug, Args)]
pub struct RangeSetArgs {
    pub marker: String,
    #[arg(long, value_enum, default_value = "reference")]
    pub kind: KindArg,
    #[arg(long, value_enum, default_value = "any")]
    pub sex: RangeSex,
    /// Lower bound of the age band (years, inclusive)
    #[arg(long, default_value_t = 0.0)]
    pub age_min: f64,
    /// Upper bound of the age band (years, exclusive)
    #[arg(long, default_value_t = 200.0)]
    pub age_max: f64,
    #[arg(long, allow_hyphen_values = true)]
    pub low: Option<f64>,
    #[arg(long, allow_hyphen_values = true)]
    pub high: Option<f64>,
    /// Unit of --low/--high (converted to the marker's canonical unit)
    #[arg(long)]
    pub unit: Option<String>,
    #[arg(long)]
    pub note: Option<String>,
    /// Set a range for this person only; it overrides the catalog ranges for them
    /// at any age (--sex/--age-min/--age-max do not apply)
    #[arg(short, long)]
    pub person: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum UnitCmd {
    /// List conversions (or known unit symbols with --symbols)
    #[command(alias = "ls")]
    List {
        /// Only conversions applicable to this marker
        #[arg(long)]
        marker: Option<String>,
        /// List unit symbols and their system instead
        #[arg(long)]
        symbols: bool,
    },
    /// Add a conversion: to = from * factor + offset
    AddConversion(AddConversionArgs),
    /// Convert a value between units
    Convert {
        #[arg(allow_hyphen_values = true)]
        value: f64,
        from: String,
        to: String,
        #[arg(long)]
        marker: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct AddConversionArgs {
    #[arg(long)]
    pub from: String,
    #[arg(long)]
    pub to: String,
    #[arg(long)]
    pub factor: f64,
    #[arg(long, default_value_t = 0.0, allow_hyphen_values = true)]
    pub offset: f64,
    /// Marker the conversion applies to (omit for a generic conversion)
    #[arg(long)]
    pub marker: Option<String>,
    /// Unit system of new unit symbols: us, si, both
    #[arg(long, default_value = "both")]
    pub system: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DedupeArg {
    Skip,
    Replace,
    Error,
}

impl DedupeArg {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Skip => "skip",
            Self::Replace => "replace",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Marker slug, alias or name
    pub marker: String,
    /// Value, optionally with a qualifier: 5.2, "<0.5", ">1000"
    #[arg(allow_hyphen_values = true)]
    pub value: String,
    /// Unit (defaults to the marker's canonical unit)
    pub unit: Option<String>,
    #[arg(short, long)]
    pub person: Option<String>,
    /// Date or datetime (default: today)
    #[arg(short, long)]
    pub date: Option<String>,
    #[arg(long)]
    pub lab: Option<String>,
    #[arg(long, overrides_with = "no_fasting")]
    pub fasting: bool,
    #[arg(long)]
    pub no_fasting: bool,
    #[arg(long)]
    pub note: Option<String>,
    #[arg(long = "tag", value_delimiter = ',')]
    pub tags: Vec<String>,
    /// What to do if a value already exists for this person/marker/time
    #[arg(long, value_enum, default_value = "error")]
    pub dedupe: DedupeArg,
}

#[derive(Debug, Args)]
pub struct RmMeasurementArgs {
    /// Measurement ids
    #[arg(required = true)]
    pub ids: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InputFormat {
    Csv,
    Tsv,
    Json,
    Jsonl,
    /// xlsx, xlsm, xlsb, xls or ods (detected from the extension)
    Spreadsheet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Layout {
    /// One row per measurement (marker, value, date columns), or one row per
    /// date with several --value-column columns
    Long,
    /// One row per date, one column per marker
    Wide,
    /// One row per marker, one column per date (dates in the header row)
    Transposed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum QualitativeArg {
    /// Count qualitative results in the summary and import nothing for them
    Skip,
    /// Store them as observations (see `biomarker observations`)
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RangesArg {
    /// Keep the catalog's reference ranges
    Catalog,
    /// Set person-specific reference ranges from the sheet's ref low/high columns
    Sheet,
}

#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Input file (`-` for stdin)
    pub file: PathBuf,
    /// Input format (default: from file extension)
    #[arg(long, value_enum)]
    pub input_format: Option<InputFormat>,
    /// Person for rows without a person column
    #[arg(short, long)]
    pub person: Option<String>,
    /// Column mapping FIELD=COLUMN (fields: person, marker, value, unit, date, time, lab, fasting, note, tags, qualifier)
    #[arg(long = "map", value_name = "FIELD=COLUMN", value_delimiter = ',')]
    pub maps: Vec<String>,
    /// TOML mapping file ([columns], [defaults], [markers]/[rename], [units], [dates], [skip], [category], ranges)
    #[arg(long, value_name = "FILE")]
    pub mapping: Option<PathBuf>,
    /// Wide layout: one row per date, one column per marker (`marker` or `marker (unit)`); same as --layout wide
    #[arg(long)]
    pub wide: bool,
    /// Default unit when a row has none (otherwise the marker's canonical unit)
    #[arg(long)]
    pub unit: Option<String>,
    /// Default lab/source
    #[arg(long)]
    pub lab: Option<String>,
    /// Extra date format for parsing (strftime)
    #[arg(long, value_name = "FMT")]
    pub input_date_format: Option<String>,
    /// Duplicate policy (default from config `dedupe`, else skip)
    #[arg(long, value_enum)]
    pub dedupe: Option<DedupeArg>,
    /// Validate and report without writing
    #[arg(long, short = 'n')]
    pub dry_run: bool,
    /// Create unknown people automatically
    #[arg(long)]
    pub create_people: bool,
    /// Create unknown markers automatically (row unit becomes canonical)
    #[arg(long)]
    pub create_markers: bool,
    /// Skip invalid rows instead of aborting the import
    #[arg(long)]
    pub skip_invalid: bool,
    /// Tag added to every imported measurement
    #[arg(long = "tag", value_delimiter = ',')]
    pub tags: Vec<String>,
    /// CSV input delimiter (default: config csv_delimiter, or tab for .tsv)
    #[arg(long, value_name = "CHAR")]
    pub input_delimiter: Option<String>,
    /// Spreadsheet: sheet to read (default: the first sheet)
    #[arg(long, value_name = "NAME")]
    pub sheet: Option<String>,
    /// Spreadsheet: list the workbook's sheets with their dimensions and exit
    #[arg(long)]
    pub list_sheets: bool,
    /// Row layout (default: long, or transposed for a spreadsheet whose header
    /// row holds several dates)
    #[arg(long, value_enum)]
    pub layout: Option<Layout>,
    /// Spreadsheet: 1-based header row (default: auto-detected)
    #[arg(long, value_name = "N")]
    pub header_row: Option<usize>,
    /// Transposed: column holding the test name (header text or letter; default: Test/Marker/Name, else the first column)
    #[arg(long, value_name = "COL")]
    pub marker_col: Option<String>,
    /// Transposed: column holding the unit (default: Units/Unit)
    #[arg(long, value_name = "COL")]
    pub unit_col: Option<String>,
    /// Transposed: column holding the reference low (default: ref. low/Low)
    #[arg(long, value_name = "COL")]
    pub ref_low_col: Option<String>,
    /// Transposed: column holding the reference high (default: ref. high/High)
    #[arg(long, value_name = "COL")]
    pub ref_high_col: Option<String>,
    /// Transposed: rows with a name but no values are section headers whose
    /// title becomes the category of markers created below them (default: on)
    #[arg(long, value_name = "BOOL", num_args = 0..=1, default_missing_value = "true")]
    pub sections_as_category: Option<bool>,
    /// Long: import a value column as a marker, SLUG=HEADER or SLUG=HEADER:UNIT
    /// (repeatable), e.g. `weight=Weight (lbs.):lb`
    #[arg(long = "value-column", value_name = "SLUG=HEADER[:UNIT]")]
    pub value_columns: Vec<String>,
    /// Non-numeric results such as "Negative" or "1+ Abnormal" (default for
    /// spreadsheets: skip; text formats treat them as invalid rows)
    #[arg(long, value_enum)]
    pub qualitative: Option<QualitativeArg>,
    /// Reference ranges from the sheet (default: catalog; mapping `ranges = "sheet"`)
    #[arg(long, value_enum)]
    pub ranges: Option<RangesArg>,
}

#[derive(Debug, Args, Clone, Default)]
pub struct FilterArgs {
    /// Person slug (repeatable). Default: config default_person, else everyone
    #[arg(short, long = "person", value_delimiter = ',')]
    pub persons: Vec<String>,
    /// Ignore default_person and include everyone
    #[arg(long)]
    pub all_people: bool,
    /// Marker slug/alias/name (repeatable or comma-separated)
    #[arg(short, long = "marker", value_delimiter = ',')]
    pub markers: Vec<String>,
    /// Marker category (repeatable)
    #[arg(short = 'c', long = "category", value_delimiter = ',')]
    pub categories: Vec<String>,
    /// From date (inclusive)
    #[arg(long, alias = "since")]
    pub from: Option<String>,
    /// To date (inclusive)
    #[arg(long, alias = "until")]
    pub to: Option<String>,
    /// Only the last N days/weeks/months/years, e.g. 90d, 1y
    #[arg(long, value_name = "DURATION")]
    pub last: Option<String>,
    #[arg(long)]
    pub lab: Option<String>,
    #[arg(long)]
    pub tag: Option<String>,
    /// Import batch id
    #[arg(long)]
    pub batch: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortKey {
    Date,
    Person,
    Marker,
    Category,
    Value,
}

#[derive(Debug, Args, Clone)]
pub struct QueryArgs {
    #[command(flatten)]
    pub filter: FilterArgs,
    /// Only values outside the configured range flavor
    #[arg(long)]
    pub flagged: bool,
    /// Only the latest value per person and marker
    #[arg(long)]
    pub latest: bool,
    #[arg(long, value_enum, default_value = "date")]
    pub sort: SortKey,
    /// Reverse sort order (newest first for date)
    #[arg(long, short = 'r')]
    pub reverse: bool,
    #[arg(long, short = 'n')]
    pub limit: Option<usize>,
}

#[derive(Debug, Args)]
pub struct ObservationsArgs {
    #[command(flatten)]
    pub filter: FilterArgs,
    /// Only observations flagged abnormal
    #[arg(long)]
    pub flagged: bool,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    #[command(flatten)]
    pub filter: FilterArgs,
    /// Include id and import batch columns
    #[arg(long)]
    pub with_ids: bool,
}

#[derive(Debug, Args)]
pub struct TrendArgs {
    #[command(flatten)]
    pub filter: FilterArgs,
    /// Windows for % change, e.g. 90d,6m,1y
    #[arg(long, value_delimiter = ',', default_value = "3m,6m,1y")]
    pub windows: Vec<String>,
    /// Omit individual points from JSON output
    #[arg(long)]
    pub no_points: bool,
    /// Minimum number of points for a series to be reported
    #[arg(long, default_value_t = 1)]
    pub min_points: usize,
}

#[derive(Debug, Args)]
pub struct FlagArgs {
    #[command(flatten)]
    pub filter: FilterArgs,
    /// Only consider the latest value per person and marker
    #[arg(long)]
    pub latest: bool,
    /// Exit with status 10 when any flagged value is found
    #[arg(long)]
    pub exit_code: bool,
}

#[derive(Debug, Args)]
pub struct DiffArgs {
    /// Earlier date
    pub from: String,
    /// Later date
    pub to: String,
    #[arg(short, long)]
    pub person: Option<String>,
    #[arg(short, long = "marker", value_delimiter = ',')]
    pub markers: Vec<String>,
    #[arg(short = 'c', long = "category", value_delimiter = ',')]
    pub categories: Vec<String>,
    /// Require measurements on exactly those dates (default: latest on or before each date)
    #[arg(long)]
    pub exact: bool,
    /// Only markers whose value changed
    #[arg(long)]
    pub changed: bool,
}

#[derive(Debug, Subcommand)]
pub enum DbCmd {
    /// Print the database path
    Path,
    /// Create the database (encrypted unless --insecure-plaintext) and apply migrations
    Init {
        /// Encrypt the new database (the default; plaintext needs --insecure-plaintext)
        #[arg(long)]
        encrypt: bool,
    },
    /// Encrypt an existing plaintext database in place (verified, then the plaintext is wiped)
    Encrypt,
    /// Re-wrap the database key under a new key-encryption key
    Rekey {
        /// Source of the new key: keychain, env (BIOMARKER_NEW_KEY) or passphrase (default: key_source setting)
        #[arg(long, value_name = "SOURCE")]
        to: Option<String>,
        /// Also re-encrypt the data under a fresh data key
        #[arg(long)]
        rotate_dek: bool,
    },
    /// Cache the unlocked key in the OS keychain for a while (no more passphrase prompts)
    Unlock {
        /// How long the session lasts, e.g. 15m, 2h
        #[arg(long, default_value = "15m")]
        ttl: String,
    },
    /// End a `db unlock` session
    Lock,
    /// Apply pending migrations (or show status)
    Migrate {
        /// Only report migration status
        #[arg(long)]
        status: bool,
    },
    /// Copy the database to FILE
    Backup { dest: PathBuf },
    /// Rebuild the database file to reclaim space
    Vacuum,
    /// Run integrity and consistency checks
    Check,
    /// Row counts and schema version
    Info,
}

#[derive(Debug, Subcommand)]
pub enum AuditCmd {
    /// Show (and verify) the audit log
    Log {
        /// Only the newest N entries
        #[arg(long, short = 'n')]
        limit: Option<usize>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigCmd {
    /// Show configuration (file values, or all resolved values with --effective)
    Show {
        /// Show every setting with its resolved value and source layer
        #[arg(long, short)]
        effective: bool,
    },
    /// Set a value in the config file
    Set { key: String, value: String },
    /// Remove a value from the config file
    Unset { key: String },
    /// Print the config file path
    Path,
    /// List available settings
    Keys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Nushell,
    Elvish,
    Powershell,
}

#[derive(Debug, Args)]
pub struct CompletionsArgs {
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Debug, Args)]
pub struct ManArgs {
    /// Write one page per (sub)command into DIR instead of printing the main page
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,
}
