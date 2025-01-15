//! The module is responsible to validate parsed json input or to validate user-input over the UI.
//!
//! The datapackage standard stores [Profiles](https://datapackage.org/standard/glossary/#profile)
//! in JSON schema draft 7 - so someday in the future we want to ensure that we work with valid data
//! in respect to the given schema.

use super::{
    DataPackageNotValidated, DataResourceNotValidated, DeltaDataPackageNotValidated,
    DeltaDataResourceNotValidated,
};

/// A trait that validates the datapackage standard data structures from the [crate::datapackge::parsed] module.
///
/// This trait is used as a marker trait for a TryFrom implementation.
pub trait Validatable {
    type Validated;
    type Error;
    fn validate(self) -> Result<Self::Validated, Self::Error>;
}

pub enum ValidationError {
    InvalidName,
    InvalidURL,
    InvalidEmail,
    // ...
}

//impl std::error::Error for ValidationError {}

pub struct DataPackage(DataPackageNotValidated);
impl Validatable for DataPackageNotValidated {
    type Validated = DataPackage;
    type Error = ValidationError;

    fn validate(self) -> Result<Self::Validated, ValidationError> {
        // todo: validation check
        Ok(DataPackage(self))
    }
}

pub struct DataResource(DataResourceNotValidated);
impl Validatable for DataResourceNotValidated {
    type Validated = DataResource;
    type Error = ValidationError;

    fn validate(self) -> Result<Self::Validated, Self::Error> {
        Ok(DataResource(self))
    }
}

pub struct DeltaDataPackage(DeltaDataPackageNotValidated);
impl Validatable for DeltaDataPackageNotValidated {
    type Validated = DeltaDataPackage;
    type Error = ValidationError;

    fn validate(self) -> Result<Self::Validated, Self::Error> {
        Ok(DeltaDataPackage(self))
    }
}

pub struct DeltaDataResource(DeltaDataResourceNotValidated);
impl Validatable for DeltaDataResourceNotValidated {
    type Validated = DeltaDataResource;
    type Error = ValidationError;

    fn validate(self) -> Result<Self::Validated, Self::Error> {
        Ok(DeltaDataResource(self))
    }
}
