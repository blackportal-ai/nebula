use std::{env, path::PathBuf, str::FromStr};

use color_eyre::eyre::Report;
use directories::ProjectDirs;
use tonic::transport::Server;

use lazy_static::lazy_static;
use tracing::{info, level_filters::LevelFilter};

use nebula_common::{
    configuration::{
        registry::get_configuration,
        tracing::{AppDefaultValuesFromEnv, initialize_logging},
    },
    server::{NebulaPackageQueryMockImpl, NebulaPackageQueryServer},
    storage::root_folder::RootFolderSource,
};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let env_vars = AppDefaultValuesFromEnv {
        proj_name: PROJECT_NAME.clone(),
        data_folder: get_data_dir(),
        config_folder: get_config_dir(),
        log_env: LOG_ENV.clone(),
        log_file: LOG_FILE.clone(),
        crate_name: env!("CARGO_CRATE_NAME").to_string(),
    };

    initialize_logging(Some(LevelFilter::INFO), env_vars)?;

    let config = get_configuration()?;
    let app_conf = config.application;

    let addr = format!("{}:{}", app_conf.host, app_conf.port).parse()?;

    let path = config.root_folder.expect("Root Folder Source expected").path.to_string();
    let ds = RootFolderSource::new_from_folder(PathBuf::from_str(path.as_str()).unwrap());
    let registry = NebulaPackageQueryMockImpl::new(ds);

    info!("{}", version());
    info!("Nebula Registry v0.1.0 - running on: '{}'", addr);
    Server::builder().add_service(NebulaPackageQueryServer::new(registry)).serve(addr).await?;

    Ok(())
}

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        env::var(format!("{}_DATA", PROJECT_NAME.clone())).ok().map(PathBuf::from);
    pub static ref CONFIG_FOLDER: Option<PathBuf> =
        env::var(format!("{}_CONFIG", PROJECT_NAME.clone())).ok().map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

pub fn get_data_dir() -> PathBuf {
    let directory = if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };
    directory
}

pub fn get_config_dir() -> PathBuf {
    let directory = if let Some(s) = CONFIG_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    };
    directory
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "blackportal.ai", env!("CARGO_PKG_NAME"))
}

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn version() -> String {
    let author = clap::crate_authors!();

    let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{current_exe_path} - {VERSION_MESSAGE}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}
