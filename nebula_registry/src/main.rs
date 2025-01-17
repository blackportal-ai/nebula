use tonic::transport::Server;

use nebula_common::{
    configuration::registry::get_configuration,
    server::{NebulaPackageQueryMockImpl, NebulaPackageQueryServer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration()?;
    let app_conf = config.application;

    let addr = format!("{}:{}", app_conf.host, app_conf.port).parse()?;
    let registry = NebulaPackageQueryMockImpl::default();

    println!("Nebula Registry v0.1.0 - running on: '{}'", addr);
    Server::builder().add_service(NebulaPackageQueryServer::new(registry)).serve(addr).await?;

    Ok(())
}
