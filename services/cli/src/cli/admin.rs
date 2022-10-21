use clap::{Parser, Subcommand};

use super::CliBackend;

#[derive(Subcommand, Debug)]
pub enum CliAdmin {
    /// Restart server
    Restart(CliAdminRestart),
    /// Manager server config
    #[clap(subcommand)]
    Config(CliConfig),
}

#[derive(Parser, Debug)]
pub struct CliAdminRestart {
    /// Server restart delay seconds (default: 3)
    #[clap(long)]
    pub delay: Option<u64>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Subcommand, Debug)]
pub enum CliConfig {
    /// Update server config and restart
    Update(CliConfigUpdate),
    /// View server config
    View(CliConfigView),
}

#[derive(Parser, Debug)]
pub struct CliConfigUpdate {
    /// New server config
    #[clap(long)]
    pub config: String,

    /// Server restart delay seconds (default: 3)
    #[clap(long)]
    pub delay: Option<u64>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliConfigView {
    #[clap(flatten)]
    pub backend: CliBackend,
}
