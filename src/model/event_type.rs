use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{
  postgres::{PgRow, PgTypeInfo},
  Decode, FromRow, Postgres, Row, Type,
};
use strum::{Display, EnumString};

#[derive(Deserialize, Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EventType {
  Unknown,
  Post,
  Boost,
}

impl<'r> FromRow<'r, PgRow> for EventType {
  fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
    let tt: String = row.try_get("template_type")?;

    match EventType::from_str(&tt) {
      Ok(t) => Ok(t),
      Err(_) => Err(sqlx::Error::TypeNotFound { type_name: tt.clone() }),
    }
  }
}

impl<'r> Decode<'r, Postgres> for EventType {
  fn decode(value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
    let s = match value.as_str() {
      Ok(s) => s,
      Err(e) => return Err(e),
    };

    match EventType::from_str(s) {
      Ok(t) => Ok(t),
      Err(e) => Err(Box::new(e)),
    }
  }
}

impl Type<Postgres> for EventType {
  fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
    PgTypeInfo::with_name("VARCHAR")
  }
}
