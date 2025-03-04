use color_eyre::eyre::{self, Report};
use tracing::info;

use crate::{
    NebulaCliState,
    client::list_packages,
    datapackage::{DataPackage, DataPackageNotValidated, ValidateData},
    registry::{FieldOptions, PackageInfo},
    storage::MetaDataSource,
};

pub struct SyncRe {}

pub struct SyncArgs {
    // todo: use timestamp datatype
    pub last_sync: Option<f32>,
}

fn from_json(pi: &PackageInfo) -> Result<DataPackage, Report> {
    let data_package: DataPackageNotValidated = serde_json::from_str(pi.datapackage_json())?;

    data_package.validate().map_err(|e| eyre::Report::msg(e.to_string()))
}

pub async fn sync_packages(_args: SyncArgs, state: &mut NebulaCliState) -> Result<SyncRe, Report> {
    // todo: use timestamp and server side decisions instead of complete list
    let fo = FieldOptions { include_datapackage_json: true, include_preview_images: false };
    let package_list = list_packages(Some(fo), state.client().unwrap()).await?;

    for pi in package_list.packages {
        // get datapackage json from package info:
        match from_json(&pi) {
            Ok(dp) => {
                if let Err(err) = state.put_package_metadata(&dp).await {
                    println!("{:?}", err);
                    continue;
                } else {
                    info!("Synced: {}", dp.name.clone().unwrap_or("nameless".into()));
                }
            }
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        }
    }

    Ok(SyncRe {})
}
