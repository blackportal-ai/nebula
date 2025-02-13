//! Contains the model internally used by nebula
//!
//!

pub mod pb_mapper;

pub type PackageType = super::registry::PackageType;

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
    package_type: PackageType,
}

impl Default for FilterSettings {
    fn default() -> Self {
        Self { package_type: crate::registry::PackageType::Both }
    }
}
