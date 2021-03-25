use gzlib::proto::sku_image_processer::sku_image_processer_server::*;
use std::error::Error;
use std::{env, path::Path};
use tempdir::TempDir;
use tokio::{fs::File, sync::oneshot};
use tokio::{
  fs::{copy, read_dir},
  process::Command,
};
use tokio::{
  fs::{create_dir, remove_file},
  prelude::*,
};
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
    let r = request.into_inner();
    let tmp_dir = TempDir::new("sku_image_temp")?;
    let image_path = tmp_dir.path().join(r.image_id);

    // 1. Store temp
    // Create image file
    let mut image_file = File::create(image_path.clone()).await?;
    // Write image bytes into it
    image_file
      .write_all(&r.image_bytes)
      .await
      .map_err(|_| Status::internal("Error while writing new file bytes into file"))?;

    // 2. Resize, produce result images
    let child = Command::new("./resize_script").arg(image_path).spawn();
    // Make sure our child succeeded in spawning and process the result
    let _ = child
      .map_err(|_| Status::internal("Error while processing and resizing SKU images"))?
      .await?;

    // 3. Create sku dir if not exist yet
    create_dir(&format!("data/{}", r.sku))
      .await
      .map_err(|_| Status::internal("Error while create sku directory in static space ALL"))?;

    // 4. Copy files to static server
    let mut d = read_dir(tmp_dir.path().join("output")).await.map_err(|_| {
      Status::internal(
        "Error while loading temp dir output folder; during image resize bg process.",
      )
    })?;

    while let Some(e) = d.next_entry().await? {
      copy(
        e.path(),
        Path::new("data").join(r.sku.to_string()).join(e.path()),
      )
      .await?;
    }

    Ok(Response::new(()))
  }

  async fn remove_image(
    &self,
    request: Request<proto::sku_image_processer::RemoveRequest>,
  ) -> Result<Response<()>, Status> {
    // 1. Check if image exist in static server
    // 2. If yes, remove it
    let r = request.into_inner();

    let mut d = read_dir(Path::new("data").join(r.sku.to_string()))
      .await
      .map_err(|_| {
        Status::internal("Error while loading SKU static folder; during IMG removal process.")
      })?;

    while let Some(e) = d.next_entry().await? {
      if let Some(p) = e.path().to_str() {
        if p.contains(&format!("_{}", r.image_id)) {
          remove_file(e.path())
            .await
            .map_err(|_| Status::invalid_argument("Hiba a képfájl törlésekor"))?;
        }
      }
    }

    Ok(Response::new(()))
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
