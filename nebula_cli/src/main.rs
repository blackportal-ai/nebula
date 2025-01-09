//! Command line interface to manage local nebula-environments

use clap::{Parser, Subcommand};
use nebula_common::{
    client::{init_client, list_packages},
    configuration::cli::get_configuration,
};

#[derive(Debug, Parser)]
struct CmdArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand, Clone, strum_macros::Display)]
enum Command {
    Init,

    Install,

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration()?;
    let reg_conf = config.remote_registry;
    let mut client = init_client(reg_conf.port).await?;

    let cli = CmdArgs::parse();
    match cli.cmd {
        //Command::Init => todo!(),
        //Command::Install => todo!(),
        //Command::Update => todo!(),
        //Command::Uninstall { all } => todo!(),
        Command::List { cached: _ } => println!("List: {:?}", list_packages(&mut client).await?),
        //Command::Search { cached } => todo!(),
        //Command::Explore {} => todo!(),
        //Command::Sync => todo!(),
        //Command::Registry {} => todo!(),
        _ => {
            println!("Command '{}' not yet implemented", cli.cmd)
        }
    }

    Ok(())
}
