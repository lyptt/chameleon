use crate::{
  logic::LogicErr,
  settings::{AppCdnStore, SETTINGS},
};

use actix_easy_multipart::tempfile::Tempfile;
use async_trait::async_trait;
use std::result::Result;

use super::cdn_file_store::CdnFileStore;

#[async_trait]
pub trait CdnStore {
  async fn upload_file(&self, local_path: &Tempfile, remote_path: &str) -> Result<String, LogicErr>;
}

pub struct Cdn {
  imp: Box<dyn CdnStore + 'static>,
}

impl Cdn {
  pub fn new() -> Cdn {
    match SETTINGS.cdn.file_store {
      AppCdnStore::Local => Cdn {
        imp: Box::new(CdnFileStore {}),
      },
    }
  }

  pub async fn upload_file(&self, local_file: &Tempfile, remote_path: &str) -> Result<String, LogicErr> {
    self.imp.upload_file(local_file, remote_path).await
  }
}
