mod app;
mod asana;
mod config;
mod events;
mod render;
mod state;

use anyhow::Result;
use app::App;
use clap::{App as ClapApp, Arg};
use config::Config;

/// Parse command and start app with corresponding configuration.
///
#[tokio::main]
async fn main() -> Result<()> {
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

    let matches = clap_app.get_matches();
    let mut config = Config::new();
    config.load(matches.value_of("config"))?;

    App::start(config).await?;
    Ok(())
}
