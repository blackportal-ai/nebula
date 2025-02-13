use super::Site;

use color_eyre::eyre::{Report, eyre};

use crate::{NebulaCliState, client::search_packages, server::PackageInfo};

pub struct SearchArgs {
    pub query: String,
    pub location: Site,
}

pub async fn search_package(
    args: SearchArgs,
    state: &mut NebulaCliState,
) -> Result<Vec<PackageInfo>, Report> {
    match args.location {
        Site::Local => Err(eyre!("Local search not implemented yet")),
        Site::Remote => {
            let tmp = search_packages(state.client()?, args.query).await?;
            Ok(tmp.packages)
        }
    }
}
