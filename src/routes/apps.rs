use actix_web::{web, HttpResponse, Responder};

use crate::{
  db::app_repository::AppPool,
  helpers::core::map_api_err,
  logic::app::{create_app, NewApp},
  model::response::ObjectResponse,
};

pub async fn api_create_app(apps: web::Data<AppPool>, new_app: web::Json<NewApp>) -> impl Responder {
  match create_app(&apps, &new_app).await {
    Ok(app) => HttpResponse::Ok().json(ObjectResponse { data: app }),
    Err(err) => map_api_err(err),
  }
}
