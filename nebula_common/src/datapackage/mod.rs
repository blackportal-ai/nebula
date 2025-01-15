//! This module contains a rust implementation for the datapackage standard
//!
//! The main types are [DataPackage] and [DataResource]. The common extensions for tables
//! are omitted for now. The delta extension to the data package standard is implemented in the
//! structs [DeltaDataPackage] and [DeltaDataResource].
//!
//! We use the module [parse] for parsing based on [serde]. The [validated] module contains the
//! types that are valid in respect to the schema of the data package standard.

mod delta;
mod parse;
mod validated;

pub use delta::DeltaDataPackageNotValidated;
pub use delta::DeltaDataResourceNotValidated;
pub use parse::DataPackageNotValidated;
pub use parse::DataResourceNotValidated;

pub use validated::DataPackage;
pub use validated::DataResource;
pub use validated::DeltaDataPackage;
pub use validated::DeltaDataResource;
