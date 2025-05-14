use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg(long)]
    pub watch_path: String,

    #[arg(long)]
    pub cloud_key: String,
}

pub fn load() -> Config {
    Config::parse()
}
