use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, strum_macros::EnumString, strum_macros::Display)]
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

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

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
