use gzlib::proto::sku_image_processer::sku_image_processer_server::*;
use sku_imgprocesser_microservice::*;
use std::env;
use std::error::Error;
use tokio::sync::oneshot;
use tonic::{transport::Server, Request, Response, Status};

use gzlib::proto;
struct SkuImageService;

impl SkuImageService {
  pub fn init() -> Self {
    Self {}
  }
}

#[tonic::async_trait]
impl SkuImageProcesser for SkuImageService {
  async fn add_image(
    &self,
    request: Request<proto::sku_image_processer::AddRequest>,
  ) -> Result<Response<()>, Status> {
    todo!()
  }

  async fn remove_image(
    &self,
    request: Request<proto::sku_image_processer::RemoveRequest>,
  ) -> Result<Response<()>, Status> {
    todo!()
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let addr = env::var("SERVICE_ADDR_SKU_IMG_PROCESSER")
    .unwrap_or("[::1]:50100".into())
    .parse()
    .unwrap();

  // Create shutdown channel
  let (tx, rx) = oneshot::channel();

  // Spawn the server into a runtime
  tokio::task::spawn(async move {
    Server::builder()
      .add_service(SkuImageProcesserServer::new(SkuImageService::init()))
      .serve_with_shutdown(addr, async {
        let _ = rx.await;
      })
      .await
      .unwrap()
  });

  tokio::signal::ctrl_c().await?;

  println!("SIGINT");

  // Send shutdown signal after SIGINT received
  let _ = tx.send(());

  Ok(())
}
