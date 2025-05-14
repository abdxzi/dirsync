mod config;
mod daemon;
mod watcher;
mod cloud;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(long)]
        path: String,
        #[arg(long)]
        api_key: String,
    },
    Daemon,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { path, api_key } => {
            crate::config::add_watch(path, api_key);
        }
        Commands::Daemon => {
            crate::daemon::start_all();
        }
    }
}
