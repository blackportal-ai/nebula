//! Functionality for listing packages

use color_eyre::eyre::{Report, eyre};

use crate::{
    NebulaCliState,
    datapackage::DataPackage,
    model::{
        FieldSettings, FilterSettings, PackageStatus, PackageType, PagationSettings, SortSettings,
    },
};

use super::state::DataSourceError;

pub struct ListArgs {
    /// status of the package: all, (non)-installed, updateable
    pub package_status: PackageStatus,

    /// type of package: dataset, model or both
    pub package_type: PackageType,
}

pub struct ListResult {
    pub packages: Vec<DataPackage>,
}

pub async fn list_packages(
    args: ListArgs,
    state: &mut NebulaCliState,
) -> Result<ListResult, Report> {
    let reval: Result<Vec<DataPackage>, DataSourceError> = state
        .apply_data_source(async move |ds| {
            let filter = FilterSettings { package_type: args.package_type };
            let sort = SortSettings::default();
            let pagation = PagationSettings::default();
            let fields = FieldSettings::default();

            Ok(ds.list_packages(sort, filter, pagation, fields).await)
        })
        .await;
    match reval {
        Ok(packages) => Ok(ListResult { packages }),
        Err(_err) => Err(eyre!("Error")),
    }
}
