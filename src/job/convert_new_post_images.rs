use blurhash::encode;
use image::imageops::FilterType;
use image::GenericImageView;
use std::path::Path;
use tempfile::TempDir;
use uuid::Uuid;

use crate::cdn::cdn_store::Cdn;
use crate::db::job_repository::JobPool;
use crate::db::post_attachment_repository::PostAttachmentPool;
use crate::db::post_repository::PostPool;
use crate::helpers::api::{map_db_err, map_ext_err};
use crate::logic::LogicErr;
use crate::model::job::{JobStatus, NewJob};
use crate::model::queue_job::{QueueJob, QueueJobType};
use crate::work_queue::queue::Queue;

pub async fn convert_new_post_images(
  job_id: Uuid,
  jobs: &JobPool,
  posts: &PostPool,
  post_attachments: &PostAttachmentPool,
  cdn: &Cdn,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let job = match jobs.fetch_optional_by_id(&job_id).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  let post = posts.fetch_by_id(&post_id).await?;
  let attachments = post_attachments.fetch_by_post_id(&post_id).await?;

  let tmp_dir = TempDir::new().map_err(map_ext_err)?;

  for attachment in attachments {
    let ext = match &attachment.content_type {
      Some(t) => match mime_guess::get_mime_extensions_str(t) {
        Some(e) => {
          let mut values = Vec::from_iter(e.iter().map(|v| (*v).to_owned()));
          values.sort_by(|a, b| {
            if a.len() < b.len() {
              std::cmp::Ordering::Less
            } else if a.len() < b.len() {
              std::cmp::Ordering::Greater
            } else {
              std::cmp::Ordering::Equal
            }
          });
          if let Some(value) = values.last() {
            value.to_owned()
          } else {
            return Err(LogicErr::InternalError(
              "File extension not found for content type associated to post attachment for post associated to job"
                .to_string(),
            ));
          }
        }
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

    let uri = format!("/{}", storage_ref);
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

    let (width, height) = image.dimensions();
    let (thumb_width, thumb_height) = thumb.dimensions();

    let blurhash = encode(4, 3, thumb_width, thumb_height, &thumb.to_rgba8().into_vec());

    let mut new_attachment = attachment.clone();
    new_attachment.width = width.try_into().unwrap_or_default();
    new_attachment.height = height.try_into().unwrap_or_default();
    new_attachment.uri = Some(uri);
    new_attachment.blurhash = Some(blurhash);

    post_attachments.update_attachment(new_attachment).await?;
  }

  let job_id = jobs
    .create(NewJob {
      created_by_id: Some(post.user_id),
      status: JobStatus::NotStarted,
      record_id: Some(post.post_id),
      associated_record_id: None,
    })
    .await
    .map_err(map_db_err)?;

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::CreatePostEvents)
    .build();

  queue.send_job(job).await?;

  Ok(())
}
