use tonic::transport::Server;

use nebula_common::{
    configuration::registry::get_configuration,
    server::{NebulaRegistryImpl, NebulaRegistryServer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration()?;
    let app_conf = config.application;

    let addr = format!("{}:{}", app_conf.host, app_conf.port).parse()?;
    let registry = NebulaRegistryImpl::default();

    Server::builder().add_service(NebulaRegistryServer::new(registry)).serve(addr).await?;

    Ok(())
}
