mod config;

use anyhow::Result;
use clap::{App as ClapApp, Arg};
use config::Config;

fn main() -> Result<()> {
    // Specify command-line app info
    let clap_app = ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Specify path to configuration file.")
                .takes_value(true),
        );

    // Parse provided command and parameters
    let matches = clap_app.get_matches();

    // Load configuration
    let mut config = Config::new();
    config.load(matches.value_of("config"))?;

    // Success since no preceding errors
    Ok(())
}
