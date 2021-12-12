use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, stdin, Write},
    path::{Path, PathBuf},
};

const FILE_NAME: &str = "config.yml";
const DEFAULT_DIRECTORY_PATH: &str = ".config/asana-tui";
const AUTHORIZATION_INSTRUCTIONS: &'static [&'static str] = &[
    "Visit the Asana Developer App Console at `https://app.asana.com/`",
    "Log in with the account you want to authorize",
    "Create a personal access token",
    "Copy and paste the token into the input below",
];

/// Oversees management of configuration file.
///
pub struct Config {
    pub access_token: Option<String>,
    file_path: Option<PathBuf>,
}

/// Define specification for configuration file.
///
#[derive(Serialize, Deserialize)]
struct FileSpec {
    pub access_token: String,
}

impl Config {
    /// Return a new empty instance.
    ///
    pub fn new() -> Config {
        Config {
            file_path: None,
            access_token: None,
        }
    }

    /// Try to load an existing configuration from the disk using the custom
    /// path if provided. If the file cannot be loaded, authorize with the
    /// user and initialize the configuration file with the new token at the
    /// default file path or the custom path if provided.
    ///
    pub fn load(&mut self, custom_path: Option<&str>) -> Result<()> {
        // Use default path unless custom path provided
        let dir_path = match custom_path {
            Some(path) => Path::new(&path).to_path_buf(),
            None => Config::default_path()?,
        };

        // Try to create dir path if it doesn't exist
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        // Specify config file path
        self.file_path = Some(dir_path.join(Path::new(FILE_NAME)));
        let file_path = self.file_path.as_ref().unwrap();

        // If file exists, try to extract token
        if file_path.exists() {
            let contents = fs::read_to_string(&file_path)?;
            let data: FileSpec = serde_yaml::from_str(&contents)?;
            self.access_token = Some(data.access_token);
        }
        // Otherwise authorize with user and create file
        else {
            self.access_token = Some(Config::authorize_with_user()?);
            self.create_file()?;
        }

        Ok(())
    }

    /// Attempt to serialize the configuration data and write it to the disk,
    /// returning any unrecoverable errors.
    ///
    fn create_file(&self) -> Result<()> {
        let data = FileSpec {
            access_token: self.access_token.clone().unwrap(),
        };
        let content = serde_yaml::to_string(&data)?;
        let file_path = self.file_path.as_ref().unwrap();
        let mut file = fs::File::create(file_path)?;
        write!(file, "{}", content)?;
        Ok(())
    }

    /// Print the authorization instructions and return the personal access
    /// token captured from stdin or an error if reading from stdin failed.
    ///
    fn authorize_with_user() -> Result<String> {
        println!("\n{}\n", env!("CARGO_PKG_NAME"));
        println!("Authorizing with Asana:\n");
        AUTHORIZATION_INSTRUCTIONS
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                println!("    {}. {}", index + 1, item);
            });
        println!();

        let mut access_token = String::new();
        print!("Token >> ");
        let _ = io::stdout().flush();
        stdin().read_line(&mut access_token)?;
        Ok(String::from(access_token.trim()))
    }

    /// Returns the path buffer for the default path to the configuration file
    /// or an error if the home directory could not be found.
    ///
    fn default_path() -> Result<PathBuf> {
        match dirs::home_dir() {
            Some(home) => {
                let home_path = Path::new(&home);
                let default_config_path = Path::new(DEFAULT_DIRECTORY_PATH);
                Ok(home_path.join(default_config_path))
            }
            None => Err(anyhow!("Failed to find $HOME directory")),
        }
    }
}
