use std::{error::Error, fmt::Display};

pub mod post;
pub mod user;

#[derive(Debug)]
pub enum LogicErr {
  // NotFound,
  DbError(sqlx::Error),
  InternalError(String),
}

impl Error for LogicErr {}

impl Display for LogicErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      // LogicErr::NotFound => f.write_str("NotFound"),
      LogicErr::DbError(err) => f.write_str(&err.to_string()),
      LogicErr::InternalError(err) => f.write_fmt(format_args!("InternalError {}", err)),
    }
  }
}
