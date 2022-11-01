use clap::{Parser, Subcommand};

use crate::cli::CliBackend;

#[derive(Subcommand, Debug)]
pub enum CliServer {
    /// Ping server
    Ping(CliPing),
    /// Server version
    Version(CliVersion),
    /// Restart server
    Restart(CliRestart),
    /// Manager server config
    #[clap(subcommand)]
    Config(CliConfig),
}

#[derive(Parser, Debug)]
pub struct CliPing {
    /// Backend host URL (default https://api.bencher.dev)
    #[clap(long)]
    pub host: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CliVersion {
    /// Backend host URL (default https://api.bencher.dev)
    #[clap(long)]
    pub host: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CliRestart {
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
