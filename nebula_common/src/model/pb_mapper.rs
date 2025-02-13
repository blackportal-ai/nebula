//! Contains functionality to map protobuf related types to nebulas internal model

use crate::{datapackage::DataPackage, registry::PackageInfo};

use super::{FilterSettings, PackageType, PagationSettings, SortSettings};

/// Maps self to Pagation Settings
pub trait PagationMapper {
    fn as_pagation(&self) -> Result<PagationSettings, Box<dyn std::error::Error>>;
}

/// Maps self to Filter Settings
pub trait FilterMapper {
    fn as_filter(&self) -> Result<FilterSettings, Box<dyn std::error::Error>>;
    fn into_filter(self) -> Result<FilterSettings, Box<dyn std::error::Error>>;
}

pub trait SortMapper {
    fn as_sort(&self) -> Result<SortSettings, Box<dyn std::error::Error>>;
}

impl PagationMapper for super::super::registry::ListPackagesRequest {
    fn as_pagation(&self) -> Result<PagationSettings, Box<dyn std::error::Error>> {
        let mut reval = PagationSettings::default();
        if let Some(limit) = self.limit {
            reval.limit = limit as u32;
        }
        if let Some(offset) = self.offset {
            reval.offset = offset as u32;
        }
        Ok(reval)
    }
}

impl PagationMapper for super::super::registry::SearchPackagesRequest {
    fn as_pagation(&self) -> Result<PagationSettings, Box<dyn std::error::Error>> {
        let mut reval = PagationSettings::default();
        if let Some(limit) = self.limit {
            reval.limit = limit as u32;
        }
        if let Some(offset) = self.offset {
            reval.offset = offset as u32;
        }
        Ok(reval)
    }
}

impl FilterMapper for super::super::registry::PackageRequest {
    fn as_filter(&self) -> Result<FilterSettings, Box<dyn std::error::Error>> {
        let mut reval = FilterSettings::default();
        if let Some(pt) = self.package_type {
            reval.package_type = PackageType::try_from(pt).unwrap();
        }
        Ok(reval)
    }

    fn into_filter(self) -> Result<FilterSettings, Box<dyn std::error::Error>> {
        self.as_filter()
    }
}

impl FilterMapper for super::super::registry::SearchPackagesRequest {
    fn as_filter(&self) -> Result<FilterSettings, Box<dyn std::error::Error>> {
        Ok(FilterSettings::default())
    }

    fn into_filter(self) -> Result<FilterSettings, Box<dyn std::error::Error>> {
        Ok(FilterSettings::default())
    }
}

impl SortMapper for super::super::registry::SearchPackagesRequest {
    fn as_sort(&self) -> Result<SortSettings, Box<dyn std::error::Error>> {
        Ok(SortSettings::default())
    }
}

impl From<DataPackage> for PackageInfo {
    fn from(val: DataPackage) -> PackageInfo {
        let mut inner = val.into_inner();
        PackageInfo {
            name: match inner.name.take() {
                Some(v) => v,
                None => "No name".to_string(),
            },
            version: match inner.version.take() {
                Some(v) => v,
                None => "0.1.0".to_string(),
            },
            description: match inner.description.take() {
                Some(v) => v,
                None => "No Description".to_string(),
            },
            license: {
                let mut reval = String::new();
                for lic in &inner.licenses {
                    reval += lic.name.as_str();
                }
                if reval.is_empty() {
                    reval = "UKNOWN".to_string()
                }
                reval
            },
            ..Default::default()
        }
    }
}
