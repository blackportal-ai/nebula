//! Delta/Nebula specifc extension to [datapackage standard](https://datapackage.org/standard/data-package/)
//!
//! Nebula add an extension to both the datapackage itself and the resources.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeltaDataPackageNotValidated {}

#[derive(Serialize, Deserialize)]
pub struct DeltaDataResourceNotValidated {}
