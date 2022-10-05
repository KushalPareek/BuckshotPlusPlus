use log::{debug, error, info, warn, LevelFilter};

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();

    debug!("Hello, world!");
    error!("Hello, world!");
    info!("Hello, world!");
    warn!("Hello, world!");
}
