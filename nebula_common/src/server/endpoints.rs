//! Contains the entry points for the grpc endpoints
//!
//! An endpoint has to implement an autogenerated trait of grpc.

use crate::model::pb_mapper::FilterMapper as _;
use crate::model::pb_mapper::PagationMapper as _;
use crate::model::{FieldSettings, FilterSettings, SortSettings};
use crate::storage::MetaDataSource;

use super::nebula_package_query_server::NebulaPackageQuery;
use super::{ListPackagesRequest, PackageInfo, PackageList, PackageRequest, SearchPackagesRequest};

use tonic::{Request, Response, Status};
use tracing::instrument;

#[derive(Debug)]
pub struct NebulaPackageQueryMockImpl<T>
where
    T: MetaDataSource + Send + Sync,
{
    inner_ds: T,
}

impl<T> NebulaPackageQueryMockImpl<T>
where
    T: MetaDataSource + Send + Sync,
{
    pub fn new(ds: T) -> Self {
        Self { inner_ds: ds }
    }
}

#[tonic::async_trait]
impl<T> NebulaPackageQuery for NebulaPackageQueryMockImpl<T>
where
    T: MetaDataSource + Send + Sync + 'static,
{
    #[instrument(name = "Get Package Info", skip(self))]
    async fn get_package_info(
        &self,
        request: Request<PackageRequest>,
    ) -> Result<Response<PackageInfo>, Status> {
        let mut package = self
            .inner_ds
            .get_package(&request.get_ref().search_query, request.get_ref().as_filter().unwrap())
            .await;

        let pi: PackageInfo = package.take().unwrap().into();
        Ok(Response::new(pi))

        /*
        match package {
            Some(package) => Ok(Response::new()),
            None => Err(Status::internal("Error handling not implemented")),
        }
        */
    }

    #[instrument(name = "List Packages", skip(self))]
    async fn list_packages(
        &self,
        request: Request<ListPackagesRequest>,
    ) -> Result<Response<PackageList>, Status> {
        let pagation = request.get_ref().as_pagation().unwrap();
        let fields = FieldSettings::default();
        let filter = FilterSettings::default();
        let sort = SortSettings::default();

        let body = self.inner_ds.list_packages(sort, filter, pagation, fields).await;
        let len = body.len();
        let body = PackageList {
            packages: body.into_iter().map(|el| el.into()).collect(),
            total_count: len as i32,
            limit: None,
            offset: None,
        };

        Ok(Response::new(body))
    }

    #[instrument(name = "Search Packages", skip(self))]
    async fn search_packages(
        &self,
        _request: Request<SearchPackagesRequest>,
    ) -> Result<Response<PackageList>, Status> {
        Err(Status::internal("not implemented"))
    }
}
