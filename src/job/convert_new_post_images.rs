use blurhash::encode;
use futures_util::future::join_all;
use image::{GenericImageView, ImageFormat};
use sqlx::{Pool, Postgres};
use tempfile::TempDir;
use uuid::Uuid;

use crate::cdn::cdn_store::Cdn;
use crate::helpers::api::{map_db_err, map_ext_err};
use crate::logic::LogicErr;
use crate::model::job::{Job, JobStatus, NewJob};
use crate::model::post::Post;
use crate::model::queue_job::{QueueJob, QueueJobType};
use crate::settings::SETTINGS;
use crate::work_queue::queue::Queue;
use log::{debug, warn};

pub async fn convert_new_post_images(
  job_id: Uuid,
  db: &Pool<Postgres>,
  cdn: &Cdn,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let job = match Job::fetch_optional_by_id(&job_id, db).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  let mut post = match Post::find_optional_by_id(&post_id, db).await {
    Some(post) => post,
    None => return Err(LogicErr::InternalError("Post not found for job".to_string())),
  };

  let storage_ref = match &post.content_image_storage_ref {
    Some(storage_ref) => storage_ref,
    None => {
      return Err(LogicErr::InternalError(
        "Post storage ref not found for job".to_string(),
      ))
    }
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

  let tmp_large_path = match tmp_dir
    .path()
    .join(Uuid::new_v4().to_string())
    .into_os_string()
    .into_string()
  {
    Ok(path) => path,
    Err(_) => {
      return Err(LogicErr::InternalError(
        "Failed to build temporary large path".to_string(),
      ))
    }
  };

  let tmp_medium_path = match tmp_dir
    .path()
    .join(Uuid::new_v4().to_string())
    .into_os_string()
    .into_string()
  {
    Ok(path) => path,
    Err(_) => {
      return Err(LogicErr::InternalError(
        "Failed to build temporary medium path".to_string(),
      ))
    }
  };

  let tmp_small_path = match tmp_dir
    .path()
    .join(Uuid::new_v4().to_string())
    .into_os_string()
    .into_string()
  {
    Ok(path) => path,
    Err(_) => {
      return Err(LogicErr::InternalError(
        "Failed to build temporary small path".to_string(),
      ))
    }
  };

  let large_file_name = format!("media/{}/lg/{}", post.user_id, Uuid::new_v4());
  let medium_file_name = format!("media/{}/md/{}", post.user_id, Uuid::new_v4());
  let small_file_name = format!("media/{}/sm/{}", post.user_id, Uuid::new_v4());

  cdn.download_file(storage_ref, &tmp_original_path).await?;

  {
    let output = tokio::process::Command::new(&SETTINGS.app.imagemagick_exe_path)
      .arg(&tmp_original_path)
      .arg("-gravity")
      .arg("center")
      .arg("-scale")
      .arg("2048x2048^")
      .arg("-extent")
      .arg("2048x2048")
      .arg("-quality")
      .arg("80")
      .arg(format!("JPEG:{}", &tmp_large_path))
      .output()
      .await
      .map_err(map_ext_err)?;

    debug!("{}", String::from_utf8_lossy(output.stderr.as_slice()));

    if !output.status.success() {
      return Err(LogicErr::InternalError("Failed to convert image".to_string()));
    }
  }

  {
    let output = tokio::process::Command::new(&SETTINGS.app.imagemagick_exe_path)
      .arg(&tmp_original_path)
      .arg("-gravity")
      .arg("center")
      .arg("-scale")
      .arg("1024x1024^")
      .arg("-extent")
      .arg("1024x1024")
      .arg("-quality")
      .arg("80")
      .arg(format!("JPEG:{}", &tmp_medium_path))
      .output()
      .await
      .map_err(map_ext_err)?;

    debug!("{}", String::from_utf8_lossy(output.stderr.as_slice()));

    if !output.status.success() {
      return Err(LogicErr::InternalError("Failed to convert image".to_string()));
    }
  }

  {
    let output = tokio::process::Command::new(&SETTINGS.app.imagemagick_exe_path)
      .arg(&tmp_original_path)
      .arg("-gravity")
      .arg("center")
      .arg("-scale")
      .arg("256x256^")
      .arg("-extent")
      .arg("256x256")
      .arg("-quality")
      .arg("80")
      .arg(format!("JPEG:{}", &tmp_small_path))
      .output()
      .await
      .map_err(map_ext_err)?;

    debug!("{}", String::from_utf8_lossy(output.stderr.as_slice()));

    if !output.status.success() {
      return Err(LogicErr::InternalError("Failed to convert image".to_string()));
    }
  }

  let mime_type = mime::JPEG.to_string();

  let upload_tasks = vec![
    cdn.upload_file(&tmp_large_path, &mime_type, &large_file_name),
    cdn.upload_file(&tmp_medium_path, &mime_type, &medium_file_name),
    cdn.upload_file(&tmp_small_path, &mime_type, &small_file_name),
  ];

  let results = join_all(upload_tasks).await;

  for result in results {
    if result.is_err() {
      return Err(result.unwrap_err());
    }
  }

  let blurhash = match image::io::Reader::open(&tmp_small_path) {
    Ok(mut img) => {
      img.set_format(ImageFormat::Jpeg);
      match img.decode() {
        Ok(img) => {
          let (width, height) = img.dimensions();
          Some(encode(4, 3, width, height, &img.to_rgba8().into_vec()))
        }
        Err(err) => {
          warn!(
            "Failed to open small image to generate blurhash, blurhash generation will be skipped: {}",
            err
          );
          None
        }
      }
    }
    Err(err) => {
      warn!(
        "Failed to open small image to generate blurhash, blurhash generation will be skipped: {}",
        err
      );
      None
    }
  };

  post.content_type_large = Some(mime::JPEG.to_string());
  post.content_type_medium = Some(mime::JPEG.to_string());
  post.content_type_small = Some(mime::JPEG.to_string());
  post.content_width_large = Some(2048);
  post.content_height_large = Some(2048);
  post.content_width_medium = Some(1024);
  post.content_height_medium = Some(1024);
  post.content_width_small = Some(256);
  post.content_height_small = Some(256);
  post.content_image_uri_large = Some(large_file_name);
  post.content_image_uri_medium = Some(medium_file_name);
  post.content_image_uri_small = Some(small_file_name);
  post.content_blurhash = blurhash;

  post.update_post_content(db).await.map_err(map_ext_err)?;

  let job_id = Job::create(
    NewJob {
      created_by_id: Some(post.user_id),
      status: JobStatus::NotStarted,
      record_id: Some(post.post_id),
      associated_record_id: None,
    },
    db,
  )
  .await
  .map_err(map_db_err)?;

  let job = QueueJob {
    job_id,
    job_type: QueueJobType::CreateEvents,
  };

  queue.send_job(job).await?;

  Ok(())
}
