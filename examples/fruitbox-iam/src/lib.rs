pub mod app;
mod auth;
mod ctx;
mod endpoint;
mod permission;
mod proto;
mod role;
mod user;
pub mod util;

use endpoint::grpc::grpc_serve;

pub async fn run() -> ultimate::Result<()> {
  grpc_serve()?.await
}
