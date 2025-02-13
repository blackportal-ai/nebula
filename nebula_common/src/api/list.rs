use color_eyre::eyre::{Report, eyre};

use crate::{NebulaCliState, client::list_packages, server::PackageInfo};

use super::Site;

pub struct ListArgs {
    pub site: Site,
}

pub async fn list_package(
    args: ListArgs,
    state: &mut NebulaCliState,
) -> Result<Vec<PackageInfo>, Report> {
    match args.site {
        Site::Local => Err(eyre!("local search not supported yet")),
        Site::Remote => {
            let package_list = list_packages(state.client().unwrap()).await?;
            Ok(package_list.packages)
        }
    }
}
