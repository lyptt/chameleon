use actix_web::HttpResponse;

use crate::logic::LogicErr;

use super::types::ApiError;

pub fn build_api_err(code: u16, reason: String, cause: Option<String>) -> HttpResponse {
  match code {
    400 => HttpResponse::BadRequest().json(ApiError { code, reason, cause }),
    401 => HttpResponse::Unauthorized().json(ApiError { code, reason, cause }),
    500 => HttpResponse::InternalServerError().json(ApiError { code, reason, cause }),
    _ => HttpResponse::NotFound().json(ApiError { code, reason, cause }),
  }
}

pub fn map_api_err(err: LogicErr) -> HttpResponse {
  match err {
    LogicErr::DbError(err) => HttpResponse::InternalServerError().json(ApiError {
      code: 500,
      reason: err,
      cause: None,
    }),
    LogicErr::UnauthorizedError => HttpResponse::Unauthorized().finish(),
    LogicErr::InternalError(err) => HttpResponse::InternalServerError().json(ApiError {
      code: 500,
      reason: err,
      cause: None,
    }),
    LogicErr::InvalidOperation(err) => HttpResponse::BadRequest().json(ApiError {
      code: 400,
      reason: err,
      cause: None,
    }),
    LogicErr::MissingRecord => HttpResponse::NotFound().finish(),
  }
}

pub fn build_api_not_found(cause: String) -> HttpResponse {
  build_api_err(404, "Resource not found".to_string(), Some(cause))
}
