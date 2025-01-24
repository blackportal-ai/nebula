use uuid::Uuid;

use crate::datapackage::DataPackage;

pub(crate) type PackageId = Uuid;

/// Trait to receive package information from a data source like the filesystem or a database
pub trait DataSource {
    fn list_packages(&self) -> Vec<PackageId>;

    fn get_package(&self, id: PackageId) -> Option<DataPackage>;
}
