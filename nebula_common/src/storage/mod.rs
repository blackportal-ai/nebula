pub mod root_folder;

use async_trait::async_trait;
use color_eyre::eyre::Report;

use crate::{
    datapackage::DataPackage,
    model::{FieldSettings, FilterSettings, PagationSettings, SortSettings},
};

/// Trait to receive package meta information from a data source like the filesystem or a database
#[async_trait]
pub trait MetaDataSource: std::fmt::Debug {
    async fn list_packages(
        &self,
        sort: SortSettings,
        filter: FilterSettings,
        pagation: PagationSettings,
        fields: FieldSettings,
    ) -> Vec<DataPackage>;

    async fn get_package(&self, query: &str, filter: FilterSettings) -> Option<DataPackage>;

    async fn search_package(
        &self,
        search_query: &str,
        sort: SortSettings,
        filter: FilterSettings,
        pagation: PagationSettings,
    ) -> Vec<DataPackage>;

    async fn put_package_metadata(&mut self, package: &DataPackage) -> Result<(), Report>;
}
