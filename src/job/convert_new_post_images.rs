use blurhash::encode;
use futures_util::future::join_all;
use image::imageops::FilterType;
use image::GenericImageView;
use std::path::Path;
use std::pin::Pin;
use tempfile::TempDir;
use uuid::Uuid;

use crate::cdn::cdn_store::Cdn;
use crate::db::job_repository::JobPool;
use crate::db::post_attachment_repository::PostAttachmentPool;
use crate::helpers::api::map_ext_err;
use crate::logic::LogicErr;
use crate::model::post_attachment::PostAttachment;

async fn convert_new_post_image(
  attachment: PostAttachment,
  tmp_dir: &TempDir,
  post_attachments: &PostAttachmentPool,
  cdn: &Cdn,
) -> Result<(), LogicErr> {
  let ext = match &attachment.content_type {
    Some(t) => match mime2ext::mime2ext(t) {
      Some(e) => e.to_owned(),
      None => {
        return Err(LogicErr::InternalError(
          "File extension not found for content type associated to post attachment for post associated to job"
            .to_string(),
        ))
      }
    },
    None => {
      return Err(LogicErr::InternalError(
        "Content Type not found for post associated to job".to_string(),
      ))
    }
  };

  let storage_ref = match &attachment.storage_ref {
    Some(storage_ref) => storage_ref,
    None => {
      return Err(LogicErr::InternalError(
        "Post storage ref not found for job".to_string(),
      ))
    }
  };

  let tmp_original_path = tmp_dir
    .path()
    .join(Uuid::new_v4().to_string())
    .into_os_string()
    .into_string()
    .map_err(|_| LogicErr::InternalError("Failed to build temporary download path".to_string()))?;
  let tmp_original_path = format!("{}.{}", tmp_original_path, ext);

  cdn.download_file(storage_ref, &tmp_original_path).await?;

  let image = image::open(Path::new(&tmp_original_path)).map_err(map_ext_err)?;
  let thumb = image.resize_to_fill(64, 64, FilterType::Nearest);
  let (thumb_width, thumb_height) = thumb.dimensions();

  let blurhash = encode(4, 3, thumb_width, thumb_height, &thumb.to_rgba8().into_vec());

  let mut new_attachment = attachment.clone();
  new_attachment.blurhash = Some(blurhash);

  post_attachments.update_attachment(new_attachment).await?;

  Ok(())
}

pub async fn convert_new_post_images(
  job_id: Uuid,
  jobs: &JobPool,
  post_attachments: &PostAttachmentPool,
  cdn: &Cdn,
) -> Result<(), LogicErr> {
  let job = match jobs.fetch_optional_by_id(&job_id).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  let attachments = post_attachments.fetch_by_post_id(&post_id).await?;

  let tmp_dir = TempDir::new().map_err(map_ext_err)?;

  // This type is complex, yes, but also unavoidable due to the types we have to work with here
  #[allow(clippy::type_complexity)]
  let mut futures: Vec<
    Pin<Box<dyn futures_util::Future<Output = std::result::Result<(), LogicErr>> + std::marker::Send>>,
  > = vec![];

  for attachment in attachments {
    futures.push(Box::pin(convert_new_post_image(
      attachment,
      &tmp_dir,
      post_attachments,
      cdn,
    )));
  }

  let results = join_all(futures).await;

  for result in results {
    if let Err(err) = result {
      log::error!("Failed to upload attachment: {}", err);
    }
  }

  Ok(())
}
