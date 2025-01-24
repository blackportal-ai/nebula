//! Contains functions that use [crate::storage::DataSource] to fetch data from a storage backend

use super::{PackageInfo, PackageList};
use crate::{datapackage::DataPackage, storage::data_source::DataSource};

#[derive(Debug, strum_macros::Display)]
pub enum FetchError {
    NotFound,
    Internal,
}

impl std::error::Error for FetchError {}

pub(super) fn fetch_packages<T>(
    _ds: &T,
    _search_query: &str,
    _package_type: i32,
) -> Result<PackageInfo, FetchError> {
    todo!();
}

fn convert_to_package_info(data: DataPackage) -> Result<PackageInfo, FetchError> {
    let info = PackageInfo {
        name: data.name.clone().unwrap(),
        version: match &data.version {
            Some(v) => v.clone(),
            None => "0.1.0".to_string(),
        },
        description: match &data.description {
            Some(v) => v.clone(),
            None => "No Information".to_string(),
        },
        license: {
            let mut reval = String::new();
            for lic in &data.licenses {
                reval += lic.name.as_str();
            }
            if reval.is_empty() {
                reval = "UKNOWN".to_string()
            }
            reval
        },
        ..Default::default()
    };

    Ok(info)
}

pub(super) fn fetch_list_packages<T>(
    ds: &T,
    _package_type: i32,
    _limit: i32,
    _offset: i32,
) -> PackageList
where
    T: DataSource,
{
    let ids = ds.list_packages();
    let mut packages = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(package) = ds.get_package(id) {
            packages
                .push(convert_to_package_info(package).expect("valid packages in data sources"));
        }
    }

    let len = packages.len();
    PackageList { packages, total_count: len as i32, limit: None, offset: None }
}
