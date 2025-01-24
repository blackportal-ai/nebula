//! The server-side implementation of the nebula-registry RPC protocol

pub use super::nebula_proto::nebula_package_query_server::NebulaPackageQueryServer;

#[allow(unused)]
pub(crate) use super::nebula_proto::*;

pub mod endpoints;
pub mod metadata_fetchers;
pub use endpoints::NebulaPackageQueryMockImpl;
