//! The module is responsible to validate [pod] of datapackage to ensure they fulfill the underlying schema.
//!
//! The datapackage standard stores [Profiles](https://datapackage.org/standard/glossary/#profile)
//! in JSON schema draft 7 - so someday in the future we want to ensure that we work with valid data
//! in respect to the given schema.
//!
//! Not implemented yet.

use std::ops::Deref;

use super::{
    DataPackageNotValidated, DataResourceNotValidated, DeltaDataPackageNotValidated,
    DeltaDataResourceNotValidated,
};

#[allow(dead_code)]
#[derive(Debug, strum_macros::Display)]
pub enum ValidationError {
    InvalidName,
    InvalidID,
    InvalidURL,
    InvalidEmail,
    InvalidResources,
    // ...
}

impl std::error::Error for ValidationError {}

/// A trait that validates the datapackage standard from structures of the [crate::datapackge::parsed] module.
///
///
pub trait ValidateData {
    type Validated;
    fn validate(self) -> Result<Self::Validated, ValidationError>;
}

/// A wrapper typ that marks input data as validated
pub struct Validated<T: Sized>(T);
impl<T: Sized> Validated<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Don't use this method, use [ValidateData::validate] instead.
    pub fn unchecked_from(from: T) -> Self {
        Validated(from)
    }
}

impl<T> Deref for Validated<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// A data package that has it's schema validated (validation not implemented yet)
pub type DataPackage = Validated<DataPackageNotValidated>;
impl Validated<DataPackageNotValidated> {}
impl ValidateData for DataPackageNotValidated {
    type Validated = DataPackage;

    fn validate(self) -> Result<Self::Validated, ValidationError> {
        if self.resources.is_empty() {
            return Err(ValidationError::InvalidResources);
        }

        // todo: remaining validation checks
        Ok(DataPackage::unchecked_from(self))
    }
}

/// A data resource that has it's schema validated (validation not implemented yet)
pub type DataResource = Validated<DataResourceNotValidated>;
impl ValidateData for DataResourceNotValidated {
    type Validated = DataResource;

    fn validate(self) -> Result<Self::Validated, ValidationError> {
        // todo: remaining validation checks
        Ok(DataResource::unchecked_from(self))
    }
}

/// A delta data package extension that has it's schema validated (validation not implemented yet)
pub type DeltaDataPackage = Validated<DeltaDataPackageNotValidated>;
impl ValidateData for DeltaDataPackageNotValidated {
    type Validated = DeltaDataPackage;

    fn validate(self) -> Result<Self::Validated, ValidationError> {
        // todo: remaining validation checks
        Ok(DeltaDataPackage::unchecked_from(self))
    }
}

/// A delta data resource extension that has it's schema validated (validation not implemented yet)
pub type DeltaDataResource = Validated<DeltaDataResourceNotValidated>;
impl ValidateData for DeltaDataResourceNotValidated {
    type Validated = DeltaDataResource;

    fn validate(self) -> Result<Self::Validated, ValidationError> {
        // todo: remaining validation checks
        Ok(DeltaDataResource::unchecked_from(self))
    }
}
