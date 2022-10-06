use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

mod http;
mod utils;
mod watcher;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    path: String,

    #[arg(long)]
    port: Option<u16>,

    #[arg(long)]
    debug: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let port = args.port.unwrap_or(8080);
    let debug = args.debug.unwrap_or(0);

    Builder::new()
        .filter_level(if debug == 0 {
            LevelFilter::Info
        } else {
            LevelFilter::Trace
        })
        .init();

    http::main(port);
    watcher::main();
}
