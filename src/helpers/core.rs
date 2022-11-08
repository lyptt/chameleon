use actix_web::HttpResponse;

use super::types::ApiError;

pub fn build_api_not_found(cause: String) -> HttpResponse {
  HttpResponse::NotFound().json(ApiError {
    code: 404,
    reason: "Resource not found".to_string(),
    cause: Some(cause),
  })
}

pub fn build_api_err(code: u16, reason: String, cause: Option<String>) -> HttpResponse {
  HttpResponse::NotFound().json(ApiError { code, reason, cause })
}
