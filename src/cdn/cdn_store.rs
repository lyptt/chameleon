use crate::{
  logic::LogicErr,
  settings::{AppCdnStore, SETTINGS},
};

use actix_easy_multipart::tempfile::Tempfile;
use async_trait::async_trait;
use std::result::Result;

use super::{cdn_file_store::CdnFileStore, cdn_s3_store::CdnS3Store};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CdnStore {
  async fn upload_tmp_file(
    &self,
    local_path: &Tempfile,
    content_type: &str,
    remote_path: &str,
  ) -> Result<String, LogicErr>;
  async fn upload_file(&self, local_path: &str, content_type: &str, remote_path: &str) -> Result<String, LogicErr>;
  async fn download_file(&self, remote_path: &str, local_path: &str) -> Result<(), LogicErr>;
}

pub struct Cdn {
  imp: Box<dyn CdnStore + Send + Sync + 'static>,
}

impl Cdn {
  pub fn new() -> Cdn {
    match SETTINGS.cdn.file_store {
      AppCdnStore::Local => Cdn {
        imp: Box::new(CdnFileStore {}),
      },
      AppCdnStore::S3 => Cdn {
        imp: Box::new(CdnS3Store {}),
      },
    }
  }

  #[cfg(test)]
  pub fn new_inner(inner: Box<dyn CdnStore + Sync + Send>) -> Cdn {
    Cdn { imp: inner }
  }

  pub async fn upload_tmp_file(
    &self,
    local_file: &Tempfile,
    content_type: &str,
    remote_path: &str,
  ) -> Result<String, LogicErr> {
    self.imp.upload_tmp_file(local_file, content_type, remote_path).await
  }

  pub async fn upload_file(&self, local_file: &str, content_type: &str, remote_path: &str) -> Result<String, LogicErr> {
    self.imp.upload_file(local_file, content_type, remote_path).await
  }

  pub async fn download_file(&self, remote_path: &str, local_path: &str) -> Result<(), LogicErr> {
    self.imp.download_file(remote_path, local_path).await
  }
}
