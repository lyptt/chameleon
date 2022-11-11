use std::error::Error;

use strum::Display;

#[derive(Debug, Display)]
pub enum JwtSessionErr {
  InvalidDataErr,
  JwtError(jsonwebtoken::errors::Error),
}

impl Error for JwtSessionErr {}
