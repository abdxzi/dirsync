mod cloud;
mod config;
mod daemon;
mod watcher;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dirsync")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    Daemon,
}

#[derive(Subcommand)]
enum ConfigCommands {
    New {
        #[arg(long)]
        path: String,
        #[arg(long)]
        api_key: String,
    },
    Update {
        #[arg(long)]
        path: String,
        #[arg(long)]
        api_key: String,
    },
    Delete {
        #[arg(long)]
        path: String,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { command } => match command {
            ConfigCommands::New { path, api_key } => {
                config::add_watch(path, api_key);
            }
            ConfigCommands::Update { path, api_key } => {
                config::update_watch(path, api_key);
            }
            ConfigCommands::Delete { path } => {
                config::delete_watch(path);
            }
            ConfigCommands::List => {
                config::list_watches();
            }
        },
        Commands::Daemon => {
            daemon::start_all();
        }
    }
}
