use std::path::PathBuf;

use color_eyre::eyre::{Report, eyre};

use crate::{
    client::init_client,
    configuration::cli::{self, get_configuration},
    registry::nebula_package_query_client::NebulaPackageQueryClient,
};

/// The state of nebula api (client side)
#[derive(Debug, Clone, Default)]
pub struct NebulaState {
    virt_env_path: Option<PathBuf>,

    registry_path: PathBuf,

    data_folder: PathBuf,

    config_folder: PathBuf,

    query_client: Option<NebulaPackageQueryClient<tonic::transport::channel::Channel>>,

    cli_api_settings: Option<cli::Settings>,
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
