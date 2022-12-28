use blurhash::encode;
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

  let post = match posts.fetch_post(&post_id, &job.created_by_id).await? {
    Some(post) => post,
    None => return Err(LogicErr::InternalError("Post not found for job".to_string())),
  };

  let tmp_dir = TempDir::new().map_err(map_ext_err)?;
  let tmp_original_path = match tmp_dir
    .path()
    .join(Uuid::new_v4().to_string())
    .into_os_string()
    .into_string()
  {
    Ok(path) => path,
    Err(_) => {
      return Err(LogicErr::InternalError(
        "Failed to build temporary download path".to_string(),
      ))
    }
  };

  for attachment in post.attachments {
    let storage_ref = match &attachment.storage_ref {
      Some(storage_ref) => storage_ref,
      None => {
        return Err(LogicErr::InternalError(
          "Post storage ref not found for job".to_string(),
        ))
      }
    };

    let content_type = match mime_guess::from_path(storage_ref).first() {
      Some(m) => m.to_string(),
      None => return Err(LogicErr::InternalError("Unsupported file type".to_string())),
    };

    let uri = format!("/{}", storage_ref);

    cdn.download_file(storage_ref, &tmp_original_path).await?;

    let image = match image::open(Path::new(&tmp_original_path)) {
      Ok(image) => image,
      Err(_) => return Err(LogicErr::InternalError("Failed to open image".to_string())),
    };

    let (width, height) = image.dimensions();
    let blurhash = encode(4, 3, width, height, &image.to_rgba8().into_vec());

    let mut new_attachment = attachment.clone();
    new_attachment.width = width.try_into().unwrap_or_default();
    new_attachment.height = height.try_into().unwrap_or_default();
    new_attachment.uri = Some(uri);
    new_attachment.content_type = Some(content_type);
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
