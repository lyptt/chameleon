use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct UserStats {
  pub following_count: i64,
  pub followers_count: i64,
  pub following_user: bool,
  pub user_is_you: bool,
}
