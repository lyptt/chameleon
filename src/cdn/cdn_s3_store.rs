use super::cdn_store::CdnStore;
use crate::{aws::clients::S3_CLIENT, helpers::api::map_ext_err, logic::LogicErr, settings::SETTINGS};

use actix_easy_multipart::tempfile::Tempfile;
use async_trait::async_trait;
use aws_sdk_s3::{
  model::{CompletedMultipartUpload, CompletedPart},
  output::CreateMultipartUploadOutput,
  types::ByteStream,
};
use aws_smithy_http::byte_stream::Length;
use futures_util::StreamExt;
use std::fs::File;

pub struct CdnS3Store {}

impl CdnS3Store {}

const UPLOAD_CHUNK_SIZE: u64 = 1024 * 1024 * 5;

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

    let file_size = tokio::fs::metadata(local_path).await.map_err(map_ext_err)?.len();
    // let body = ByteStream::from_path(local_path).await.map_err(map_ext_err)?;

    if file_size == 0 {
      return Err(LogicErr::InvalidOperation("File is empty, aborting upload".to_string()));
    }

    let bucket_name = SETTINGS.cdn.container.clone().unwrap();

    let multipart_upload_res: CreateMultipartUploadOutput = S3_CLIENT
      .get()
      .unwrap()
      .create_multipart_upload()
      .bucket(&bucket_name)
      .key(&absolute_remote_path)
      .content_type(content_type)
      .send()
      .await
      .map_err(map_ext_err)?;

    let upload_id = match multipart_upload_res.upload_id() {
      Some(upload_id) => upload_id,
      None => {
        return Err(LogicErr::InternalError(
          "No upload ID present for multipart upload".to_string(),
        ))
      }
    };

    let mut chunk_count = (file_size / UPLOAD_CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % UPLOAD_CHUNK_SIZE;
    if size_of_last_chunk == 0 {
      size_of_last_chunk = UPLOAD_CHUNK_SIZE;
      chunk_count -= 1;
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
      let this_chunk = if chunk_count - 1 == chunk_index {
        size_of_last_chunk
      } else {
        UPLOAD_CHUNK_SIZE
      };

      let stream = ByteStream::read_from()
        .path(local_path)
        .offset(chunk_index * UPLOAD_CHUNK_SIZE)
        .length(Length::Exact(this_chunk))
        .build()
        .await
        .map_err(map_ext_err)?;

      //Chunk index needs to start at 0, but part numbers start at 1.
      let part_number = (chunk_index as i32) + 1;
      let upload_part_res = S3_CLIENT
        .get()
        .unwrap()
        .upload_part()
        .key(&absolute_remote_path)
        .bucket(&bucket_name)
        .upload_id(upload_id)
        .body(stream)
        .part_number(part_number)
        .send()
        .await
        .map_err(map_ext_err)?;

      upload_parts.push(
        CompletedPart::builder()
          .e_tag(upload_part_res.e_tag.unwrap_or_default())
          .part_number(part_number)
          .build(),
      );
    }

    let completed_multipart_upload = CompletedMultipartUpload::builder()
      .set_parts(Some(upload_parts))
      .build();

    S3_CLIENT
      .get()
      .unwrap()
      .complete_multipart_upload()
      .key(&absolute_remote_path)
      .bucket(&bucket_name)
      .multipart_upload(completed_multipart_upload)
      .upload_id(upload_id)
      .send()
      .await
      .map_err(map_ext_err)?;

    Ok(absolute_remote_path)
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

    let raw_temp_file = match File::options().read(true).write(true).create_new(true).open(local_path) {
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
