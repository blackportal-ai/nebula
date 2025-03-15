use std::{fs::exists, path::PathBuf, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, strum::EnumString, strum::Display)]
pub enum Environment {
    #[strum(ascii_case_insensitive)]
    Dev,

    #[strum(ascii_case_insensitive)]
    Installed,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub remote_registry: RegistrySettings,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RegistrySettings {
    pub port: u16,
    pub host: String,
}

fn check_candidates(base_path: &PathBuf, options: impl IntoIterator<Item: AsRef<str>>) -> Option<PathBuf> {
    for candidate in options {
        let path_candidate = base_path.join(candidate.as_ref());
        if exists(&path_candidate).unwrap_or(false)
            && exists(path_candidate.join("base.yaml")).unwrap_or(false)
        {
            return Some(path_candidate)
        }
    }

    None
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    // cargo run may be invoked from the workspace, so we have a candidate for the configuration path.
    let options = ["configuration", "nebula_cli/configuration"];
    let candidate = check_candidates(&base_path, options);
    if candidate.is_none()  {
        return Err(config::ConfigError::NotFound(format!("with base path: {}", base_path.to_str().unwrap_or("/?"))));
    }
    //~

    let configuration_directory = candidate.unwrap();
    let env = std::env::var("NEBULA_CLI_ENVIRONMENT").unwrap_or_else(|_| "dev".to_owned());
    let env = Environment::from_str(env.as_str()).expect("Failed to parse NEBULA_CLI_ENVIRONMENT");
    let conf_file = format! {"{}.yaml", env.to_string().to_lowercase()};

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")))
        .add_source(config::File::from(configuration_directory.join(conf_file)))
        .add_source(
            config::Environment::with_prefix("NEBULA_CLI").prefix_separator("_").separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
