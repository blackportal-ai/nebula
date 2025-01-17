//! Client calls to nebula-registry endpoints

use tonic::Request;
use tonic::transport::Channel;

use crate::server::{ListPackagesRequest, PackageType};

use super::nebula_proto::nebula_package_query_client::NebulaPackageQueryClient;
use super::nebula_proto::{PackageInfo, PackageList, PackageRequest, SearchPackagesRequest};

pub async fn init_client(
    host: String,
    port: u16,
) -> Result<NebulaPackageQueryClient<Channel>, Box<dyn std::error::Error>> {
    let client = NebulaPackageQueryClient::connect(format!("http://{}:{}", host, port)).await?;
    Ok(client)
}

pub async fn list_packages(
    client: &mut NebulaPackageQueryClient<Channel>,
) -> Result<PackageList, Box<dyn std::error::Error>> {
    let request = Request::new(ListPackagesRequest {
        field_options: None,
        package_type: PackageType::Both as i32,
        sort: None,
        limit: Some(30),
        offset: None,
    });
    let response = client.list_packages(request).await?;
    Ok(response.into_inner())
}

pub async fn get_package_info(
    client: &mut NebulaPackageQueryClient<Channel>,
    name: String,
) -> Result<PackageInfo, Box<dyn std::error::Error>> {
    let request =
        Request::new(PackageRequest { search_query: name, package_type: PackageType::Both as i32 });
    let response = client.get_package_info(request).await?;

    Ok(response.into_inner())
}

pub async fn search_packages(
    client: &mut NebulaPackageQueryClient<Channel>,
    query: String,
) -> Result<PackageList, Box<dyn std::error::Error>> {
    let request = Request::new(SearchPackagesRequest {
        field_options: None,
        search_query: query,
        package_type: PackageType::Both as i32,
        sort: vec![],
        limit: None,
        offset: None,
        created_date: None,
        updated_date: None,
        kind: None,
        authors: vec![],
        min_downloads: None,
        max_downloads: None,
    });
    let response = client.search_packages(request).await?;

    Ok(response.into_inner())
}
