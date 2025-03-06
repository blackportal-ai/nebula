use color_eyre::eyre::Report;

use crate::{NebulaCliState, client::search_packages, registry::PackageInfo};

pub struct SearchArgs {
    pub query: String,
}

pub async fn search_package(
    args: SearchArgs,
    state: &mut NebulaCliState,
) -> Result<Vec<PackageInfo>, Report> {
    let tmp = search_packages(state.client()?, args.query).await?;
    Ok(tmp.packages)
}
