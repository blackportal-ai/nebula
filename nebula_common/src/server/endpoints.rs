use super::nebula_registry_server::NebulaRegistry;
use super::{Empty, PackageInfo, PackageList, PackageRequest};

use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NebulaRegistryImpl {}

fn dummy_package() -> PackageInfo {
    PackageInfo {
        description: "Dataset image classification".to_string(),
        name: "cifar10".to_string(),
        version: "2".to_string(),
    }
}

#[tonic::async_trait]
impl NebulaRegistry for NebulaRegistryImpl {
    async fn get_package_info(
        &self,
        request: Request<PackageRequest>,
    ) -> Result<Response<PackageInfo>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(dummy_package()))
    }

    async fn list_packages(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<PackageList>, Status> {
        println!("Got a request: {:?}", request);

        let response_body = PackageList { packages: vec![dummy_package()] };

        Ok(Response::new(response_body))
    }
}
