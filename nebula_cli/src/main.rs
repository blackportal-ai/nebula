//! Command line interface to manage local nebula-environments

use clap::Parser as _;
use cli::{Cli, command_interpret};
use nebula_common::{client::init_client, configuration::cli::get_configuration};

use nebula_common::nebula_proto::nebula_package_query_client::NebulaPackageQueryClient;
use tonic::transport::Channel;

mod cli;
mod dirs;
#[cfg(feature = "tui")]
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read top-level cli:
    let args = Cli::parse();

    // read config:
    let config = get_configuration()?;
    let reg_conf = config.remote_registry;
    let client = init_client(reg_conf.host, reg_conf.port).await?;

    // start as interactive tui if asked by user
    run(args, client).await
}

#[cfg(feature = "tui")]
async fn run(
    args: Cli,
    mut client: NebulaPackageQueryClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};
    use tui::App;

    if args.tui {
        let mut app = App::new(args.tick_rate, args.frame_rate, client)?;
        app.run().await?;
    } else {
        // otherwise process one command
        command_interpret(std::env::args_os(), &mut client).await?;
    }
    Ok(())
}

#[cfg(not(feature = "tui"))]
async fn run(
    mut args: Cli,
    mut client: NebulaPackageQueryClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write as _};

    use clap::CommandFactory;

    if let Some(_initial_cmd) = args.cmd {
        command_interpret(std::env::args_os(), &mut client).await?;
    }

    println!("{}\nType 'help' or 'help <command-name>' for instructions", cli::version());
    while args.interactive {
        // Prompt user for input
        print!("> ");
        io::stdout().flush()?;

        // Read input from stdin
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        input = input.trim().to_lowercase();
        if input.is_empty() {
            continue;
        }

        let mut it = input.split(' ');
        if let Some(first) = it.next() {
            if first == "quit" || first == "exit" {
                args.interactive = false;
            } else if first == "help" {
                if let Some(sub_command) = it.next() {
                    let sub_command = sub_command.trim();
                    let mut cmd_fac = Cli::command();
                    let cmd_candidate =
                        cmd_fac.get_subcommands_mut().find(|cmd| cmd.get_name() == sub_command);
                    if let Some(cmd) = cmd_candidate {
                        cmd.print_long_help()?;
                    } else {
                        println!("Command '{}' not defined - printing help overview:", sub_command);
                        Cli::command().print_long_help()?
                    }
                } else {
                    Cli::command().print_long_help()?;
                }
            } else {
                // Convert input to iterator and use in command_interpret
                input = "nebula_cli ".to_owned() + &input;
                let args = input.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
                command_interpret(args.into_iter(), &mut client).await?;
            }
        }
    }
    Ok(())
}
