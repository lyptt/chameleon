use crate::{helpers::api::map_db_err, logic::LogicErr, model::post_attachment::PostAttachment};

use super::FromRow;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostAttachmentRepo {
  async fn create_attachment_from(&self, attachment: PostAttachment) -> Result<(), LogicErr>;
  async fn update_attachment(&self, attachment: PostAttachment) -> Result<(), LogicErr>;
  async fn fetch_by_post_id(&self, post_id: &Uuid) -> Result<Vec<PostAttachment>, LogicErr>;
}

pub type PostAttachmentPool = Arc<dyn PostAttachmentRepo + Send + Sync>;

pub struct DbPostAttachmentRepo {
  pub db: Pool,
}

#[async_trait]
impl PostAttachmentRepo for DbPostAttachmentRepo {
  async fn create_attachment_from(&self, attachment: PostAttachment) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "INSERT INTO post_attachments (attachment_id, user_id, post_id, uri, width, height, content_type, storage_ref, blurhash, created_at) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
      &[
        &attachment.attachment_id,
        &attachment.user_id,
        &attachment.post_id,
        &attachment.uri,
        &attachment.width,
        &attachment.height,
        &attachment.content_type,
        &attachment.storage_ref,
        &attachment.blurhash,
        &attachment.created_at,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn update_attachment(&self, attachment: PostAttachment) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "UPDATE post_attachments SET user_id = $2, post_id = $3, uri = $4, width = $5, height = $6, content_type = $7, storage_ref = $8, blurhash = $9, created_at = $10 WHERE attachment_id = $1",
      &[
        &attachment.attachment_id,
        &attachment.user_id,
        &attachment.post_id,
        &attachment.uri,
        &attachment.width,
        &attachment.height,
        &attachment.content_type,
        &attachment.storage_ref,
        &attachment.blurhash,
        &attachment.created_at,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn fetch_by_post_id(&self, post_id: &Uuid) -> Result<Vec<PostAttachment>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query("SELECT * FROM post_attachments WHERE post_id = $1", &[&post_id])
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(PostAttachment::from_row).collect())
  }
}
