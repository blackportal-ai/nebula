//! Client calls to nebula-registry endpoints

use tonic::transport::Channel;

use super::nebula_proto::nebula_registry_client::NebulaRegistryClient;
use super::nebula_proto::{Empty, PackageInfo, PackageList, PackageRequest};

pub async fn init_client(
    port: u16,
) -> Result<NebulaRegistryClient<tonic::transport::Channel>, Box<dyn std::error::Error>> {
    let client = NebulaRegistryClient::connect(format!("http://[127.0.0.1]:{}", port)).await?;
    Ok(client)
}

pub async fn list_packages(
    client: &mut NebulaRegistryClient<Channel>,
) -> Result<PackageList, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(Empty {});
    let response = client.list_packages(request).await?;
    Ok(response.into_inner())
}

pub async fn get_package_info(
    client: &mut NebulaRegistryClient<Channel>,
    name: String,
) -> Result<PackageInfo, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(PackageRequest { name });
    let response = client.get_package_info(request).await?;

    Ok(response.into_inner())
}
