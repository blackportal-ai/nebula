use color_eyre::eyre::Report;
use nebula_common::NebulaCliState;

use crate::cli::{Cli, run_legacy_cmd};

pub async fn run_tui(args: Cli, state: NebulaCliState) -> Result<(), Report> {
    use super::App;

    if args.tui {
        let mut app = App::new(args.tick_rate, args.frame_rate, state)?;
        app.run().await?;
    } else {
        // otherwise process one command
        run_legacy_cmd(args, state).await?;
    }
    Ok(())
}
