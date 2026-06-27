use std::sync::OnceLock;

use clap::Parser;

static ARGS: OnceLock<Args> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub directory: Option<String>,
}

impl Args {
    pub fn get() -> &'static Self {
        ARGS.get_or_init(Args::parse)
    }
}
