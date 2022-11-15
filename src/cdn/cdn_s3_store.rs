use super::cdn_store::CdnStore;
use crate::{aws::clients::S3_CLIENT, helpers::api::map_ext_err, logic::LogicErr, settings::SETTINGS};

use actix_easy_multipart::tempfile::Tempfile;
use async_trait::async_trait;
use aws_sdk_s3::types::ByteStream;
use futures_util::StreamExt;
use std::fs::File;

pub struct CdnS3Store {}

impl CdnS3Store {}

#[async_trait]
impl CdnStore for CdnS3Store {
  async fn upload_tmp_file(&self, local_path: &Tempfile, remote_path: &str) -> Result<String, LogicErr> {
    let absolute_remote_path = match SETTINGS.cdn.path.is_empty() {
      true => remote_path.to_string(),
      false => format!("{}/{}", SETTINGS.cdn.path, remote_path),
    };

    let body = ByteStream::from_path(local_path.file.path()).await;
    S3_CLIENT
      .get()
      .unwrap()
      .put_object()
      .bucket(SETTINGS.cdn.container.clone().unwrap())
      .key(&absolute_remote_path)
      .body(body.unwrap())
      .content_type(
        local_path
          .content_type
          .clone()
          .unwrap_or(mime::APPLICATION_OCTET_STREAM)
          .to_string(),
      )
      .send()
      .await
      .map_err(map_ext_err)?;

    tokio::fs::remove_file(local_path.file.path())
      .await
      .map(|_| Ok(absolute_remote_path))
      .map_err(map_ext_err)?
  }

  async fn upload_file(&self, local_path: &str, content_type: &str, remote_path: &str) -> Result<String, LogicErr> {
    let absolute_remote_path = match SETTINGS.cdn.path.is_empty() {
      true => remote_path.to_string(),
      false => format!("{}/{}", SETTINGS.cdn.path, remote_path),
    };

    let body = ByteStream::from_path(local_path).await;
    S3_CLIENT
      .get()
      .unwrap()
      .put_object()
      .bucket(SETTINGS.cdn.container.clone().unwrap())
      .key(&absolute_remote_path)
      .body(body.unwrap())
      .content_type(content_type)
      .send()
      .await
      .map_err(map_ext_err)?;

    tokio::fs::remove_file(local_path)
      .await
      .map(|_| Ok(absolute_remote_path))
      .map_err(map_ext_err)?
  }

  async fn download_file(&self, remote_path: &str, local_path: &str) -> Result<(), LogicErr> {
    let absolute_remote_path = match SETTINGS.cdn.path.is_empty() {
      true => remote_path.to_string(),
      false => format!("{}/{}", SETTINGS.cdn.path, remote_path),
    };

    let response = match S3_CLIENT
      .get()
      .unwrap()
      .get_object()
      .bucket(SETTINGS.cdn.container.clone().unwrap())
      .key(&absolute_remote_path)
      .send()
      .await
    {
      Ok(res) => res,
      Err(err) => return Err(map_ext_err(err)),
    };

    let mut byte_stream = response.body;

    let raw_temp_file = match File::options()
      .read(true)
      .write(true)
      .create_new(true)
      .open(&local_path)
    {
      Ok(file) => file,
      Err(err) => return Err(map_ext_err(err)),
    };

    let mut temp_file = tokio::fs::File::from(raw_temp_file);

    while let Some(item) = byte_stream.next().await {
      tokio::io::copy(&mut item.map_err(map_ext_err)?.as_ref(), &mut temp_file)
        .await
        .map_err(map_ext_err)?;
    }

    Ok(())
  }
}
