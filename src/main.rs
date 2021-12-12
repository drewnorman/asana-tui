mod asana;
mod config;
mod state;

use anyhow::{anyhow, Result};
use asana::Asana;
use clap::{App as ClapApp, Arg};
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
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

    // Try to get access token from configuration
    let access_token = config
        .access_token
        .ok_or(anyhow!("failed to retrieve access token"))?
        .clone();

    // Initialize Asana client
    let mut asana = Asana::new(access_token);

    // Request current user data
    println!("{:?}", asana.me().await);

    // Success since no preceding errors
    Ok(())
}
