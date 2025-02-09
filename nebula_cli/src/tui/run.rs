use color_eyre::eyre::Report;
use nebula_common::nebula_proto::nebula_package_query_client::NebulaPackageQueryClient;
use tonic::transport::Channel;

use crate::cli::{Cli, run_legacy_cmd};

pub async fn run_tui(args: Cli, client: NebulaPackageQueryClient<Channel>) -> Result<(), Report> {
    use super::App;

    if args.tui {
        let mut app = App::new(args.tick_rate, args.frame_rate, client)?;
        app.run().await?;
    } else {
        // otherwise process one command
        run_legacy_cmd(args, client).await?;
    }
    Ok(())
}
