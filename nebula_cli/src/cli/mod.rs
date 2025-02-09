//! command line interpreting module based on [clap]
//!
//!

use std::ffi::OsString;

use clap::{Parser, Subcommand};

use tracing::{info, warn};

use color_eyre::eyre::Report;
use nebula_common::{
    client::{list_packages, search_packages},
    nebula_proto::nebula_package_query_client::NebulaPackageQueryClient,
};

use crate::version;

mod run;
pub use run::run_legacy_cmd;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[cfg(feature = "tui")]
    #[arg(long, default_value_t = false)]
    /// use a [ratatui] based terminal user interface instead of a simple cmd-tool
    pub tui: bool,

    #[arg(short, long, default_value_t = false)]
    /// start the cmd-tool in interactive mode, that allows typing multiple commands
    pub interactive: bool,

    /// use verbose output, only in non TUI mode.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    /// command that is executed
    pub cmd: Option<Command>,

    /// Tick rate, i.e. number of ticks per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]
    pub tick_rate: f64,

    /// Frame rate, i.e. number of frames per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,
}

#[derive(Debug, Parser)]
pub struct CmdArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(
    Debug,
    Subcommand,
    Clone,
    strum::Display,
    strum::EnumIter,
    strum::EnumDiscriminants,
    strum::EnumString
)]
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
) -> Result<(), Report>
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
        Command::List { cached: _ } => info!("List: {:?}", list_packages(client).await?),
        Command::Search { cached: _ } => {
            info!("Search: {:?}", search_packages(client, "cifar".into()).await?)
        }
        //Command::Explore {} => todo!(),
        //Command::Sync => todo!(),
        //Command::Registry {} => todo!(),
        _ => {
            warn!("Command '{}' not yet implemented", cli.cmd)
        }
    }

    Ok(())
}
