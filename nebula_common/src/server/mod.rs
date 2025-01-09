//! The server-side implementation of the nebula-registry RPC protocol

pub use super::nebula_proto::nebula_registry_server::NebulaRegistryServer;

#[allow(unused)]
pub(crate) use super::nebula_proto::*;

pub mod endpoints;
pub use endpoints::NebulaRegistryImpl;
