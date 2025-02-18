use color_eyre::eyre::{Report, eyre};

use crate::{
    NebulaCliState, client,
    registry::{FieldOptions, PackageInfo},
};

use super::Site;

pub struct ListArgs {
    pub site: Site,

    pub field_options: FieldOptions,
}

pub async fn list_packages(
    args: ListArgs,
    state: &mut NebulaCliState,
) -> Result<Vec<PackageInfo>, Report> {
    match args.site {
        Site::Local => Err(eyre!("local search not supported yet")),
        Site::Remote => {
            let package_list =
                client::list_packages(Some(args.field_options), state.client().unwrap()).await?;
            Ok(package_list.packages)
        }
    }
}
