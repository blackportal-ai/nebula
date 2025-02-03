//! Delta/Nebula specifc extension to [datapackage standard](https://datapackage.org/standard/data-package/)
//!
//! Nebula add an extension to both the datapackage itself and the resources.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeltaDataPackageNotValidated {
    pub category: String,
    pub classes: Option<u32>,
    pub training_count: Option<u32>,
    pub validation_count: Option<u32>,
    pub test_count: Option<u32>,
    pub input_shape: String,
    pub mirror: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeltaDataResourceNotValidated {
    pub origin: String,
    pub format: Option<String>,
    pub local_storage: String,
}
