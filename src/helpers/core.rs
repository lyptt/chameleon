use actix_web::HttpResponse;

use super::types::ApiError;

pub fn build_api_err(code: u16, reason: String, cause: Option<String>) -> HttpResponse {
  match code {
    400 => HttpResponse::BadRequest().json(ApiError { code, reason, cause }),
    401 => HttpResponse::Unauthorized().json(ApiError { code, reason, cause }),
    500 => HttpResponse::InternalServerError().json(ApiError { code, reason, cause }),
    _ => HttpResponse::NotFound().json(ApiError { code, reason, cause }),
  }
}

pub fn build_api_not_found(cause: String) -> HttpResponse {
  build_api_err(404, "Resource not found".to_string(), Some(cause))
}
