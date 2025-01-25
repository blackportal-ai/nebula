pub mod root_folder;

use std::future::Future;

use crate::{
    datapackage::DataPackage,
    model::{FieldSettings, FilterSettings, PagationSettings, SortSettings},
};

/// Trait to receive package meta information from a data source like the filesystem or a database
pub trait MetaDataSource {
    fn list_packages(
        &self,
        sort: SortSettings,
        filter: FilterSettings,
        pagation: PagationSettings,
        fields: FieldSettings,
    ) -> impl Future<Output = Vec<DataPackage>> + Send;

    fn get_package(
        &self,
        query: &str,
        filter: FilterSettings,
    ) -> impl Future<Output = Option<DataPackage>> + Send;

    fn search_package(
        &self,
        search_query: &str,
        sort: SortSettings,
        filter: FilterSettings,
        pagation: PagationSettings,
    ) -> impl Future<Output = Vec<DataPackage>> + Send;
}
