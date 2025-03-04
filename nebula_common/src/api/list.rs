//! Functionality for listing packages

use color_eyre::eyre::{Report, eyre};

use crate::{
    NebulaCliState, client,
    datapackage::DataPackage,
    model::{FieldSettings, FilterSettings, PagationSettings, SortSettings},
    registry::{FieldOptions, PackageList},
};

use super::{Site, state::DataSourceError};

pub struct ListArgs {
    pub site: Site,

    pub field_options: FieldOptions,
}

pub struct ListResult {
    pub packages: Vec<DataPackage>,

    pub client_packages: Option<PackageList>,
}

pub async fn list_packages(
    args: ListArgs,
    state: &mut NebulaCliState,
) -> Result<ListResult, Report> {
    match args.site {
        Site::Local => {
            let reval: Result<Vec<DataPackage>, DataSourceError> = state
                .apply_data_source(async |ds| {
                    let filter =
                        FilterSettings { package_type: crate::registry::PackageType::Both };
                    let sort = SortSettings::default();
                    let pagation = PagationSettings::default();
                    let fields = FieldSettings::default();

                    Ok(ds.list_packages(sort, filter, pagation, fields).await)
                })
                .await;
            match reval {
                Ok(packages) => Ok(ListResult { packages, client_packages: None }),
                Err(_err) => Err(eyre!("Error")),
            }
        }
        Site::Remote => {
            let package_list =
                client::list_packages(Some(args.field_options), state.client().unwrap()).await?;

            let packages = package_list
                .packages
                .into_iter()
                .map(|e| DataPackage::try_from(e).unwrap()) // todo: get rid of unwrap
                .collect();

            Ok(ListResult { packages, client_packages: None })
        }
    }
}
