//! CLI for cited marker panels.
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum PanelCmd {
    /// List available panels
    #[command(alias = "ls")]
    List,
    /// Show a panel and its markers
    Show { panel: String },
}
