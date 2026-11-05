use clap::Parser;
use cliculator::cli::{
    commands::commands_enum::Commands::{self},
    root::Root,
};

fn main() {
    let root = Root::parse();
    match &root.command {
        Commands::Calculate(args) => args.run(),
    }
}
