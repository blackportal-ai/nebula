pub mod client;
pub mod configuration;
pub mod server;

pub mod nebula_proto {
    tonic::include_proto!("nebula");
}
