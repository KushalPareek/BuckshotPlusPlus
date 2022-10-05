use log::{LevelFilter, debug};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    debug: Option<u8>,
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();

    let args = Args::parse();
    debug!("Path: {}", args.path);

    if let Some(debug) = args.debug {
        debug!("Debug: {:?}", debug);
    }
}
