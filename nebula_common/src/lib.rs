pub mod client;
pub mod configuration;
pub mod datapackage;
pub mod server;

pub mod nebula_proto {
    tonic::include_proto!("nebula.v1");
}

pub mod nebula_proto_fallback {
    // include older version here
}
