//! Nebula common library crate with functionality for both registry and cli

pub mod api;
pub mod client;
pub mod configuration;
pub mod datapackage;
pub mod model;
pub mod registry;
pub mod storage;

pub mod nebula_proto {
    tonic::include_proto!("nebula.v1");
}

pub mod nebula_proto_fallback {
    // include older version here
}

pub use api::state::NebulaState as NebulaCliState;
