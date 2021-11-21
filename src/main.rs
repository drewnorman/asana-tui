use clap::App as ClapApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify command-line app info
    let clap_app = ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));

    // Parse provided command and parameters
    clap_app.get_matches();

    // Success since no preceding errors
    Ok(())
}
