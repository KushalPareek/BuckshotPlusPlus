use clap::Parser;
use log::LevelFilter;

mod http;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    path: String,

    #[arg(long)]
    port: Option<i16>,

    #[arg(long)]
    debug: Option<u8>,
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();

    let args = Args::parse();
    http::main(args.port.unwrap_or(8080), args.debug.unwrap_or(0));
}
