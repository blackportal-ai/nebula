//! command line interpreting module based on [clap]
//!
//!

use std::{env, ffi::OsString};

use clap::{Parser, Subcommand};

use nebula_common::{
    client::{list_packages, search_packages},
    nebula_proto::nebula_package_query_client::NebulaPackageQueryClient,
};

use crate::config::{get_config_dir, get_data_dir};

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(long, default_value_t = false)]
    pub tui: bool,

    #[command(subcommand)]
    pub cmd: Option<Command>,

    /// Tick rate, i.e. number of ticks per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]
    pub tick_rate: f64,

    /// Frame rate, i.e. number of frames per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,
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

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{VERSION_MESSAGE}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}

#[derive(Debug, Parser)]
pub struct CmdArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand, Clone, strum::Display, strum::EnumIter, strum::EnumDiscriminants)]
#[strum_discriminants(name(CommandVariants))]
pub enum Command {
    /// init the virtual environment in the given folder
    Init {
        #[clap(long, short, action)]
        folder: Option<String>,
    },

    /// prints status information that depend on the current virtual environment
    Status,

    /// Installs a package
    Install,

    /// Updates a package
    Update,

    Uninstall {
        #[clap(long, short, action)]
        all: bool,
    },

    Search {
        #[clap(long, short, action)]
        cached: bool,
    },

    List {
        #[clap(long, short, action)]
        cached: bool,
    },

    Explore {},

    Sync,

    Registry {
        // todo subcommands
    },
}

pub async fn command_interpret<I, T>(
    itr: I,
    client: &mut NebulaPackageQueryClient<tonic::transport::channel::Channel>,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = CmdArgs::parse_from(itr);

    match cli.cmd {
        //Command::Init => todo!(),
        //Command::Install => todo!(),
        //Command::Update => todo!(),
        //Command::Uninstall { all } => todo!(),
        Command::List { cached: _ } => println!("List: {:?}", list_packages(client).await?),
        Command::Search { cached: _ } => {
            println!("Search: {:?}", search_packages(client, "cifar".into()).await?)
        }
        //Command::Explore {} => todo!(),
        //Command::Sync => todo!(),
        //Command::Registry {} => todo!(),
        _ => {
            println!("Command '{}' not yet implemented", cli.cmd)
        }
    }

    Ok(())
}
