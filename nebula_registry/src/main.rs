use std::{path::PathBuf, str::FromStr};

use color_eyre::eyre::Report;
use tonic::transport::Server;

use nebula_common::{
    configuration::registry::get_configuration,
    server::{NebulaPackageQueryMockImpl, NebulaPackageQueryServer},
    storage::root_folder::RootFolderSource,
};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let config = get_configuration()?;
    let app_conf = config.application;

    let addr = format!("{}:{}", app_conf.host, app_conf.port).parse()?;

    let path = config.root_folder.expect("Root Folder Source expected").path.to_string();
    let ds = RootFolderSource::new_from_folder(PathBuf::from_str(path.as_str()).unwrap());
    let registry = NebulaPackageQueryMockImpl::new(ds);

    println!("Nebula Registry v0.1.0 - running on: '{}'", addr);
    Server::builder().add_service(NebulaPackageQueryServer::new(registry)).serve(addr).await?;

    Ok(())
}
