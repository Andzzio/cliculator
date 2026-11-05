use clap::Subcommand;

use crate::cli::commands::args::calculate_args::CalculateArgs;
#[derive(Subcommand)]
pub enum Commands {
    Calculate(CalculateArgs),
}
