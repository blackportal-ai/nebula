use crate::registry::PackageInfo;

use super::{DataPackage, DataPackageNotValidated, ValidateData as _, validated::ValidationError};

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Json needed in PackageInfo to transform to DataPackage")]
    NoJson,

    #[error("Invalid json")]
    InvalidJson(#[from] serde_json::Error),

    #[error("Invalid DataPackage spec")]
    InvalidDataPackage(#[from] ValidationError),
}

impl TryFrom<PackageInfo> for DataPackage {
    type Error = MyError;

    fn try_from(value: PackageInfo) -> Result<Self, Self::Error> {
        if let Some(json) = value.datapackage_json {
            let ndp: DataPackageNotValidated = serde_json::from_str(json.as_str())?;
            let dp = ndp.validate()?;
            Ok(dp)
        } else {
            Err(MyError::NoJson)
        }
    }
}
