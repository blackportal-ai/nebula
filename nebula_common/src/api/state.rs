use std::{path::PathBuf, sync::Arc};

use color_eyre::eyre::{Report, eyre};
use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::{
    client::init_client,
    configuration::cli::{self, get_configuration},
    datapackage::DataPackage,
    model::{FieldSettings, FilterSettings, PagationSettings, SortSettings},
    registry::nebula_package_query_client::NebulaPackageQueryClient,
    storage::{MetaDataSource, root_folder::RootFolderSource},
};

/// The state of nebula api (client side)
///
/// todo: get rid of lifetime, e.g. by using async closures to work with [NebulaState::data_source]
#[derive(Debug)]
pub struct NebulaState {
    virt_env_path: Option<PathBuf>,

    registry_path: PathBuf,

    data_folder: PathBuf,

    config_folder: PathBuf,

    query_client: Option<NebulaPackageQueryClient<tonic::transport::channel::Channel>>,

    cli_api_settings: Option<cli::Settings>,

    data_source: Option<Arc<Mutex<Box<dyn MetaDataSource + Send + Sync>>>>,
}

impl NebulaState {
    pub fn new(data_folder: PathBuf, config_folder: PathBuf) -> Self {
        let registry_path = data_folder.join("local-registry");
        NebulaState {
            data_folder,
            config_folder,
            registry_path,

            virt_env_path: None,
            cli_api_settings: None,
            query_client: None,
            data_source: None,
        }
    }

    pub fn init_config(&mut self) -> Result<(), Report> {
        self.cli_api_settings = Some(get_configuration()?);
        Ok(())
    }

    pub async fn init_client(&mut self) -> Result<(), Report> {
        if let Some(cfg) = &self.cli_api_settings {
            let rr = &cfg.remote_registry;
            self.query_client = Some(init_client(&rr.host, rr.port).await?);
            Ok(())
        } else {
            Err(eyre!("Cannot init client: Configuration not loaded"))
        }
    }

    pub fn init_data_source(&mut self) {
        if self.data_source.is_none() {
            self.data_source = Some(Arc::new(Mutex::new(Box::new(
                RootFolderSource::new_from_folder(self.registry_path.clone()),
            ))))
        }
    }

    /// this method makes sense as soon as async closures are stablized to get rid of the lifetime parameter
    #[cfg(any())]
    pub fn apply_data_source<F: Fn(&mut dyn MetaDataSource) -> bool>(&mut self, ftor: F) -> bool {
        match &mut self.data_source {
            Some(k) => ftor(k.deref_mut()),
            None => false,
        }
    }

    pub fn data(&self) -> &PathBuf {
        &self.data_folder
    }

    pub fn config(&self) -> &PathBuf {
        &self.config_folder
    }

    pub fn registry(&self) -> &PathBuf {
        &self.registry_path
    }

    pub fn virtual_path(&self) -> &Option<PathBuf> {
        &self.virt_env_path
    }

    pub fn client(
        &mut self,
    ) -> Result<&mut NebulaPackageQueryClient<tonic::transport::channel::Channel>, Report> {
        Ok(self.query_client.as_mut().unwrap())
    }
}

#[async_trait]
impl MetaDataSource for NebulaState {
    async fn list_packages(
        &self,
        _sort: SortSettings,
        _filter: FilterSettings,
        _pagation: PagationSettings,
        _fields: FieldSettings,
    ) -> Vec<DataPackage> {
        if let Some(ds) = &self.data_source {
            let ds = ds.lock().await;
            return ds.list_packages(_sort, _filter, _pagation, _fields).await;
        } else {
            vec![]
        }
    }

    async fn get_package(&self, package: &str, _filter: FilterSettings) -> Option<DataPackage> {
        if let Some(ds) = &self.data_source {
            let ds = ds.lock().await;
            return ds.get_package(package, _filter).await;
        } else {
            None
        }
    }

    async fn search_package(
        &self,
        _search_query: &str,
        _sort: SortSettings,
        _filter: FilterSettings,
        _pagation: PagationSettings,
    ) -> Vec<DataPackage> {
        todo!()
    }

    async fn put_package_metadata(&mut self, package: &DataPackage) -> Result<(), Report> {
        if let Some(ds) = &self.data_source {
            let mut ds = ds.lock().await;
            return ds.put_package_metadata(package).await;
        } else {
            Err(eyre!("data source not ready"))
        }
    }
}
