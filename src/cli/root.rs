use clap::Parser;

use crate::cli::commands::commands_enum::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Root {
    #[command(subcommand)]
    pub command: Commands,
}
