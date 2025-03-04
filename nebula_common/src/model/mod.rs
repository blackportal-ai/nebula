//! Contains the model internally used by nebula
//!
//!

use std::ops::{Deref, DerefMut};

use num_enum::TryFromPrimitive;

pub mod pb_mapper;

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PackageStatus {
    /// all packages if installed or not
    All,

    /// all packages that are not yet installed
    NotInstalled,

    /// all packages that are installed
    #[default]
    Installed,

    /// all packages that are installed but that have updates on the remote site
    Updatedable,
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PackageType {
    #[default]
    Both,

    Dataset,

    Model,
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Source {
    /// use the local nebula registry as data source
    #[default]
    Local,

    /// use the remote nebula registry as data source
    Remote,

    /// use both sources to provide a diff, helpful to pre process updates
    Diff,
}

impl From<PackageType> for super::registry::PackageType {
    fn from(value: PackageType) -> Self {
        // safety: We keep PackageType in sync
        super::registry::PackageType::try_from((value as u8) as i32).unwrap()
    }
}

/// Optional MetaData Fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaDataField {
    PreviewImages,

    DataPackage,
    // todo more?
}

/// Settings to select additional (heavy) fields
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FieldSettings {
    optional_fields: Vec<MetaDataField>, // todo: on stack? set semantic...
}

impl Deref for FieldSettings {
    type Target = Vec<MetaDataField>;

    fn deref(&self) -> &Self::Target {
        &self.optional_fields
    }
}

impl DerefMut for FieldSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.optional_fields
    }
}

/// Pagation Settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagationSettings {
    pub limit: u32,

    pub offset: u32,
}

impl Default for PagationSettings {
    fn default() -> Self {
        Self { limit: 30, offset: Default::default() }
    }
}

/// multi level sort settinggs, not implemented
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SortSettings {}

/// Filter Settings, not implemnted
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterSettings {
    pub package_type: PackageType,
}

impl Default for FilterSettings {
    fn default() -> Self {
        Self { package_type: PackageType::Both }
    }
}
