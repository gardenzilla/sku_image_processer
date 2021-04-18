use gzlib::proto::sku_image_processer::sku_image_processer_server::*;
use std::error::Error;
use std::{env, path::Path};
use tokio::{fs::read_dir, process::Command};
use tokio::{fs::remove_file, io::AsyncWriteExt};
use tokio::{
  fs::{create_dir_all, File},
  sync::oneshot,
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

    // Create dir all ORIGINAL if necessary
    create_dir_all(Path::new("data").join(r.sku.to_string()).join("original"))
      .await
      .map_err(|_| Status::internal("Error while create sku directory in static space ALL"))?;

    // Create dir all SIZED if necessary
    create_dir_all(Path::new("data").join(r.sku.to_string()).join("sized"))
      .await
      .map_err(|_| Status::internal("Error while create sku directory in static space ALL"))?;

    let image_file_path = Path::new("data")
      .join(r.sku.to_string())
      .join("original")
      .join(r.image_id);

    // 1. Store original image file
    let mut image_file = File::create(&image_file_path).await?;

    // Write data into new image file
    image_file
      .write_all(&r.image_bytes)
      .await
      .map_err(|_| Status::internal("Error while writing new file bytes into file"))?;

    // 2. Resize, produce result images
    let child = Command::new("img_process.sh").arg(&image_file_path).spawn();

    // Make sure our child succeeded in spawning and process the result
    let _ = child
      .map_err(|_| Status::internal("Error while processing and resizing SKU images"))?
      .wait()
      .await?;

    Ok(Response::new(()))
  }

  async fn remove_image(
    &self,
    request: Request<proto::sku_image_processer::RemoveRequest>,
  ) -> Result<Response<()>, Status> {
    // 1. Check if image exist in static server
    // 2. If yes, remove it
    let r = request.into_inner();

    let mut d = read_dir(Path::new("data").join(r.sku.to_string()).join("sized"))
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
