use super::nebula_package_query_server::NebulaPackageQuery;
use super::{ListPackagesRequest, PackageInfo, PackageList, PackageRequest, SearchPackagesRequest};

use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NebulaPackageQueryMockImpl {}

fn dummy_package() -> PackageInfo {
    PackageInfo {
        description: "Dataset image classification".to_string(),
        name: "cifar10".to_string(),
        version: "2".to_string(),
        download_size: 0,
        installed_size: 0,
        license: "todo".into(),
        datapackage_json: None,
        preview_images: vec![],
    }
}

#[tonic::async_trait]
impl NebulaPackageQuery for NebulaPackageQueryMockImpl {
    async fn get_package_info(
        &self,
        request: Request<PackageRequest>,
    ) -> Result<Response<PackageInfo>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(dummy_package()))
    }

    async fn list_packages(
        &self,
        request: Request<ListPackagesRequest>,
    ) -> Result<Response<PackageList>, Status> {
        println!("Got a request: {:?}", request);

        let response_body = PackageList {
            packages: vec![dummy_package()],
            total_count: 1,
            limit: None,
            offset: None,
        };

        Ok(Response::new(response_body))
    }

    async fn search_packages(
        &self,
        _request: Request<SearchPackagesRequest>,
    ) -> Result<Response<PackageList>, Status> {
        Err(Status::internal("not implemented"))
    }
}
