use bandsaw::Bandsaw;
use log::{info, LevelFilter};

static LOGGER: Bandsaw = Bandsaw;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to set logger");

    info!("Hello World")
}
