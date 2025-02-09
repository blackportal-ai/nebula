//! Command line interface to manage local nebula-environments

use std::env;
use std::path::PathBuf;

use clap::Parser as _;
use cli::Cli;
use directories::ProjectDirs;
use nebula_common::configuration::tracing::{AppDefaultValuesFromEnv, initialize_logging};
use nebula_common::{client::init_client, configuration::cli::get_configuration};

use lazy_static::lazy_static;

use color_eyre::eyre::Report;
use tracing::level_filters::LevelFilter;

#[cfg(feature = "tui")]
use tui::run_tui;

#[cfg(not(feature = "tui"))]
use cli::run_legacy_cmd;

mod cli;
#[cfg(feature = "tui")]
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    // read top-level cli:
    let args = Cli::parse();

    let env_vars = AppDefaultValuesFromEnv {
        proj_name: PROJECT_NAME.clone(),
        data_folder: get_data_dir(),
        config_folder: get_config_dir(),
        log_env: LOG_ENV.clone(),
        log_file: LOG_FILE.clone(),
        crate_name: env!("CARGO_CRATE_NAME").to_string(),
    };

    let lvl = if args.verbose { LevelFilter::TRACE } else { LevelFilter::INFO };
    #[cfg(feature = "tui")]
    {
        initialize_logging(if args.tui { None } else { Some(lvl) }, env_vars)?;
    }
    #[cfg(not(feature = "tui"))]
    initialize_logging(Some(lvl), env_vars)?;

    // read config:
    let config = get_configuration()?;
    let reg_conf = config.remote_registry;
    let client = init_client(reg_conf.host, reg_conf.port).await?;

    #[cfg(feature = "tui")]
    {
        run_tui(args, client).await
    }
    #[cfg(not(feature = "tui"))]
    {
        run_legacy_cmd(args, client).await
    }
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
