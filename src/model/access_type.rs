use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{
  postgres::{PgRow, PgTypeInfo},
  Decode, FromRow, Postgres, Row, Type,
};
use strum::{Display, EnumString};

#[derive(Deserialize, Serialize, EnumString, Display, Debug)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
  Unknown,
  Shadow,
  Unlisted,
  Private,
  FollowersOnly,
  PublicLocal,
  PublicFederated,
}

impl<'r> FromRow<'r, PgRow> for AccessType {
  fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
    let tt: String = row.try_get("template_type")?;

    match AccessType::from_str(&tt) {
      Ok(t) => Ok(t),
      Err(_) => Err(sqlx::Error::TypeNotFound { type_name: tt.clone() }),
    }
  }
}

impl<'r> Decode<'r, Postgres> for AccessType {
  fn decode(value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
    let s = match value.as_str() {
      Ok(s) => s,
      Err(e) => return Err(e),
    };

    match AccessType::from_str(&s) {
      Ok(t) => Ok(t),
      Err(e) => Err(Box::new(e)),
    }
  }
}

impl Type<Postgres> for AccessType {
  fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
    PgTypeInfo::with_name("VARCHAR")
  }
}
