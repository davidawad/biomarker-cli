//! Subcommands for interventions, regimens and interactions (docs/interventions.md).

use std::path::PathBuf;

use clap::{Args, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RxStatusArg {
    Otc,
    Rx,
    Controlled,
    Unknown,
}

impl RxStatusArg {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Otc => "otc",
            Self::Rx => "rx",
            Self::Controlled => "controlled",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum InterventionCmd {
    /// Add a supplement, drug or other intervention to the catalog
    Add(InterventionAddArgs),
    /// List the catalog
    #[command(alias = "list")]
    Ls {
        /// Filter by category
        #[arg(long)]
        category: Option<String>,
        /// Case-insensitive substring search over slug, name, aliases and ingredients
        #[arg(long)]
        search: Option<String>,
    },
    /// Show an intervention with its interactions and regimen counts
    Show { intervention: String },
    /// Remove an intervention and its interaction edges (and, with --force, its regimens)
    Rm {
        intervention: String,
        #[arg(long)]
        force: bool,
    },
    /// Import supplement-interactions/1 JSON files (or directories of <slug>/interactions.json)
    Import(InterventionImportArgs),
}

#[derive(Debug, Args)]
pub struct InterventionAddArgs {
    /// Short identifier, e.g. `magnesium-glycinate`
    pub slug: String,
    #[arg(long)]
    pub name: Option<String>,
    /// supplement, drug, food, lifestyle, ...
    #[arg(long, default_value = "supplement")]
    pub category: String,
    #[arg(long, value_enum, default_value = "unknown")]
    pub rx_status: RxStatusArg,
    /// Ingredient NAME[=AMOUNT[UNIT]], e.g. magnesium=120mg (repeatable)
    #[arg(long = "ingredient", value_name = "NAME[=AMOUNT[UNIT]]")]
    pub ingredients: Vec<String>,
    /// Drug class, e.g. anticoagulant (repeatable or comma-separated)
    #[arg(long = "class", value_delimiter = ',')]
    pub classes: Vec<String>,
    #[arg(long = "alias", value_delimiter = ',')]
    pub aliases: Vec<String>,
    #[arg(long)]
    pub description: Option<String>,
}

#[derive(Debug, Args)]
pub struct InterventionImportArgs {
    /// Files or directories (`<dir>/*/interactions.json`)
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,
    /// Validate and report without writing
    #[arg(long, short = 'n')]
    pub dry_run: bool,
    /// Skip invalid interactions instead of aborting the import
    #[arg(long)]
    pub skip_invalid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum StatusArg {
    Taking,
    Stopped,
    Planned,
    All,
}

#[derive(Debug, Subcommand)]
pub enum RegimenCmd {
    /// Start a regimen: a person taking an intervention
    Add(RegimenAddArgs),
    /// Stop a regimen (by id, or every active regimen of an intervention)
    Stop {
        /// Regimen id or intervention
        target: String,
        #[arg(short, long)]
        person: Option<String>,
        /// First day not taken (default: today)
        #[arg(long)]
        on: Option<String>,
    },
    /// List regimens with their status (taking, stopped, planned) as of a date
    #[command(alias = "list")]
    Ls(RegimenLsArgs),
    /// Remove regimens by id
    Rm {
        #[arg(required = true)]
        ids: Vec<i64>,
    },
    /// Marker values before and after each start and stop of an intervention
    Impact(RegimenImpactArgs),
}

#[derive(Debug, Args)]
pub struct RegimenAddArgs {
    /// Intervention slug, alias or name
    pub intervention: String,
    #[arg(short, long)]
    pub person: Option<String>,
    #[arg(long)]
    pub dose: Option<f64>,
    /// Dose unit, e.g. mg, IU, capsule
    #[arg(long)]
    pub unit: Option<String>,
    /// e.g. daily, 2x/day, weekly, prn
    #[arg(long)]
    pub frequency: Option<String>,
    /// Time slot, e.g. morning, evening, bedtime, with-meals
    #[arg(long)]
    pub slot: Option<String>,
    /// First day taken (default: today)
    #[arg(long)]
    pub start: Option<String>,
    /// First day not taken
    #[arg(long)]
    pub stop: Option<String>,
    #[arg(long)]
    pub note: Option<String>,
}

#[derive(Debug, Args)]
pub struct RegimenLsArgs {
    /// Person slug (repeatable). Default: config default_person, else everyone
    #[arg(short, long = "person", value_delimiter = ',')]
    pub persons: Vec<String>,
    /// Ignore default_person and include everyone
    #[arg(long)]
    pub all_people: bool,
    /// Date the status is computed for (default: today)
    #[arg(long)]
    pub as_of: Option<String>,
    #[arg(long, value_enum, default_value = "all")]
    pub status: StatusArg,
    /// Only regimens of this intervention
    #[arg(long)]
    pub intervention: Option<String>,
}

#[derive(Debug, Args)]
pub struct RegimenImpactArgs {
    /// Intervention slug, alias or name
    pub intervention: String,
    #[arg(short, long)]
    pub person: Option<String>,
    /// Marker slug/alias/name (repeatable or comma-separated)
    #[arg(short, long = "marker", value_delimiter = ',')]
    pub markers: Vec<String>,
    /// Marker category (repeatable)
    #[arg(short = 'c', long = "category", value_delimiter = ',')]
    pub categories: Vec<String>,
    /// Days on each side of the event, e.g. 90d, 12w, 6m
    #[arg(long, default_value = "90d")]
    pub window: String,
}

#[derive(Debug, Subcommand)]
pub enum InteractionCmd {
    /// Add (or update) an interaction edge
    Add(InteractionAddArgs),
    /// List interaction edges
    #[command(alias = "list")]
    Ls(InteractionLsArgs),
    /// Contraindication matrix of a person's active regimen (x the catalog with --catalog)
    Grid {
        #[arg(short, long)]
        person: Option<String>,
        /// Day the regimen is taken from (default: today)
        #[arg(long)]
        as_of: Option<String>,
        /// Columns also include every catalog intervention with an edge to the regimen
        #[arg(long)]
        catalog: bool,
    },
    /// Remove interaction edges by id
    Rm {
        #[arg(required = true)]
        ids: Vec<i64>,
    },
}

#[derive(Debug, Args)]
pub struct InteractionAddArgs {
    /// Intervention the edge starts from
    pub intervention: String,
    /// Target: an intervention, or TYPE:slug with TYPE intervention, drug-class, condition, lab-test, genotype
    pub target: String,
    /// contraindicated, avoid, caution, monitor, synergy, lab-interference
    #[arg(long)]
    pub kind: String,
    /// major, moderate, minor, unknown
    #[arg(long, default_value = "unknown")]
    pub severity: String,
    #[arg(long)]
    pub mechanism: Option<String>,
    /// Citation (repeatable), e.g. PMID:12345 or a URL
    #[arg(long = "cite")]
    pub citations: Vec<String>,
    #[arg(long)]
    pub note: Option<String>,
}

#[derive(Debug, Args)]
pub struct InteractionLsArgs {
    /// Only edges touching this intervention (either side, or through its drug classes)
    #[arg(short, long)]
    pub intervention: Option<String>,
    /// Only these kinds (repeatable or comma-separated)
    #[arg(long = "kind", value_delimiter = ',')]
    pub kinds: Vec<String>,
    /// Only edges at least this severe (major, moderate, minor)
    #[arg(long)]
    pub min_severity: Option<String>,
    /// Only edges to this target type
    #[arg(long)]
    pub target_kind: Option<String>,
}
