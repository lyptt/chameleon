use actix_web::{web, HttpResponse, Responder};

pub async fn web_serve_static(path: web::Path<String>) -> impl Responder {
  if *path == "styles/styles.css" {
    return HttpResponse::Ok()
      .content_type("text/css")
      .body(include_str!("../../public/static/styles/styles.css"));
  }

  HttpResponse::NotFound().finish()
}
