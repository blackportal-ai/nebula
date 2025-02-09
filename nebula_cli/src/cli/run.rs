//! runner for a legecy command line

use super::{Cli, command_interpret};
use color_eyre::eyre::Report;
use nebula_common::nebula_proto::nebula_package_query_client::NebulaPackageQueryClient;
use tonic::transport::Channel;

pub async fn run_legacy_cmd(
    mut args: Cli,
    mut client: NebulaPackageQueryClient<Channel>,
) -> Result<(), Report> {
    use clap::CommandFactory;
    use color_eyre::Section as _;
    use tracing::info;

    use std::io::{self, Write as _};
    info!("{}", super::version());

    if let Some(_initial_cmd) = args.cmd {
        command_interpret(std::env::args_os(), &mut client).await?;
    } else if !args.interactive {
        // neither initial cmd nor interactive --> wrong usage
        Cli::command().print_long_help()?;

        return Err(Report::msg("Invalid command-line usage")
            .with_suggestion(|| "Either use 'nebula_cli --i' or use a command: 'nebula_cli list'")
            .with_suggestion(|| "If unsure use 'nebula_cli help'"));
    } else {
        info!("Type 'help' or 'help <command-name>' for instructions");
    }

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

    info!("Gracefully shutting down");
    Ok(())
}
