//! Command line interface to manage local nebula-environments

use app::App;
use clap::Parser as _;
use cli::{Cli, command_interpret};
use color_eyre::Result;
use nebula_common::{client::init_client, configuration::cli::get_configuration};

pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read top-level cli:
    let args = Cli::parse();

    // read config:
    let config = get_configuration()?;
    let reg_conf = config.remote_registry;
    let mut client = init_client(reg_conf.host, reg_conf.port).await?;

    // start as interactive tui if asked by user
    if args.tui {
        let mut app = App::new(args.tick_rate, args.frame_rate, client)?;
        app.run().await?;
    } else {
        // otherwise process one command
        command_interpret(std::env::args_os(), &mut client).await?;
    }
    Ok(())
}
