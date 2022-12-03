use std::error::Error;

use strum::Display;

pub mod app;
pub mod comment;
pub mod follow;
pub mod job;
pub mod like;
pub mod post;
pub mod user;

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum LogicErr {
  // NotFound,
  DbError(String),
  UnauthorizedError,
  InternalError(String),
  InvalidOperation(String),
  MissingRecord,
}

impl Error for LogicErr {}
