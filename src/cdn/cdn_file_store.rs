use super::cdn_store::CdnStore;
use crate::{helpers::handlers::map_ext_err, logic::LogicErr, settings::SETTINGS};

use actix_easy_multipart::tempfile::Tempfile;
use async_trait::async_trait;

#[derive(Clone)]
pub struct CdnFileStore {}

#[async_trait]
impl CdnStore for CdnFileStore {
  async fn upload_file(&self, local_path: &Tempfile, remote_path: &str) -> Result<String, LogicErr> {
    let absolute_remote_path = match SETTINGS.cdn.path.is_empty() {
      true => remote_path.to_string(),
      false => format!("{}/{}", SETTINGS.cdn.path, remote_path),
    };

    tokio::fs::copy(local_path.file.path(), absolute_remote_path)
      .await
      .map_err(map_ext_err)?;

    tokio::fs::remove_file(local_path.file.path())
      .await
      .map(|_| Ok(remote_path.to_string()))
      .map_err(map_ext_err)?
  }
}
