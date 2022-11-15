use std::borrow::BorrowMut;

use magick_rust::bindings::GravityType_CenterGravity;
use magick_rust::MagickWand;
use sqlx::{Pool, Postgres};
use tempfile::TempDir;
use uuid::Uuid;

use crate::cdn::cdn_store::Cdn;
use crate::helpers::api::map_ext_err;
use crate::logic::LogicErr;
use crate::model::job::Job;
use crate::model::post::Post;
use crate::model::queue_job::QueueJob;

pub async fn convert_new_post_images(job_id: Uuid, db: &Pool<Postgres>, cdn: &Cdn) -> Result<(), LogicErr> {
  let job = match Job::fetch_optional_by_id(&job_id, &db).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let post_id = match job.completion_record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  let mut post = match Post::find_optional_by_id(&post_id, &db).await {
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

  cdn.download_file(&storage_ref, &tmp_original_path).await?;

  {
    let mut wand = MagickWand::new();
    wand.read_image(&tmp_original_path).map_err(map_ext_err)?;
    wand.set_gravity(GravityType_CenterGravity).map_err(map_ext_err)?;
    wand.adaptive_resize_image(2048, 2048).map_err(map_ext_err)?;
    wand.set_format("JPEG").map_err(map_ext_err)?;
    wand.set_compression_quality(80).map_err(map_ext_err)?;

    wand.write_image(&tmp_large_path).map_err(map_ext_err)?;
  }

  {
    let mut wand = MagickWand::new();
    wand.read_image(&tmp_original_path).map_err(map_ext_err)?;
    wand.set_gravity(GravityType_CenterGravity).map_err(map_ext_err)?;
    wand.adaptive_resize_image(1024, 1024).map_err(map_ext_err)?;
    wand.set_format("JPEG").map_err(map_ext_err)?;
    wand.set_compression_quality(80).map_err(map_ext_err)?;

    wand.write_image(&tmp_medium_path).map_err(map_ext_err)?;
  }

  {
    let mut wand = MagickWand::new();
    wand.read_image(&tmp_original_path).map_err(map_ext_err)?;
    wand.set_gravity(GravityType_CenterGravity).map_err(map_ext_err)?;
    wand.adaptive_resize_image(256, 256).map_err(map_ext_err)?;
    wand.set_format("JPEG").map_err(map_ext_err)?;
    wand.set_compression_quality(80).map_err(map_ext_err)?;

    wand.write_image(&tmp_small_path).map_err(map_ext_err)?;
  }

  cdn
    .upload_file(&tmp_large_path, &mime::JPEG.to_string(), &large_file_name)
    .await?;

  cdn
    .upload_file(&tmp_medium_path, &mime::JPEG.to_string(), &medium_file_name)
    .await?;

  cdn
    .upload_file(&tmp_small_path, &mime::JPEG.to_string(), &small_file_name)
    .await?;

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

  post.update_post_content(&db).await.map_err(map_ext_err)?;

  Ok(())
}
