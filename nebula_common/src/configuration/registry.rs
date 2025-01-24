use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, strum_macros::EnumString, strum_macros::Display)]
pub enum Environment {
    #[strum(ascii_case_insensitive)]
    Local,

    #[strum(ascii_case_insensitive)]
    Production,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub root_folder: Option<RootFolder>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    //#[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RootFolder {
    pub path: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let env = std::env::var("NEBULA_REG_ENVIRONMENT").unwrap_or_else(|_| "local".to_owned());
    let env = Environment::from_str(env.as_str()).expect("Failed to parse NEBULA_REG");
    let conf_file = format! {"{}.yaml", env.to_string().to_lowercase()};

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")))
        .add_source(config::File::from(configuration_directory.join(conf_file)))
        .add_source(
            config::Environment::with_prefix("NEBULA_REG").prefix_separator("_").separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
