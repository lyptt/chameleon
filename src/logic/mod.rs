use std::{error::Error, fmt::Display};

use strum::Display;

pub mod post;
pub mod user;

#[derive(Debug, Display)]
pub enum LogicErr {
  // NotFound,
  DbError(sqlx::Error),
  UnauthorizedError,
  InternalError(String),
}

impl Error for LogicErr {}
