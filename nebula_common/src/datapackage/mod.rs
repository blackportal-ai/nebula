//! This module contains a rust implementation for the datapackage standard
//!
//! The main types are [DataPackage] and [DataResource]. The common extensions for tables
//! are omitted for now. The delta extension to the data package standard is implemented in the
//! structs [DeltaDataPackage] and [DeltaDataResource].
//!
//! We use the module [pod] for parsing based on [serde]. The [validated] module contains the
//! types that are valid in respect to the schema of the data package standard.

mod delta;
mod pod;
mod validated;

use std::path::Path;

pub use delta::DeltaDataPackageNotValidated;
pub use delta::DeltaDataResourceNotValidated;
pub use pod::DataPackageNotValidated;
pub use pod::DataResourceNotValidated;
pub use pod::datapackage_meta_from_file_not_validated;

pub use validated::DataPackage;
pub use validated::DataResource;
pub use validated::DeltaDataPackage;
pub use validated::DeltaDataResource;
use validated::ValidateData;

/// Reads a json file that contains the datapackage descriptor as json and checks it validity
///
/// filepath: A path on the filesystem
pub fn datapackage_meta_from_file(filepath: &Path) -> Result<DataPackage, std::io::Error> {
    let not_validated = datapackage_meta_from_file_not_validated(filepath)?;
    not_validated
        .validate()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}
