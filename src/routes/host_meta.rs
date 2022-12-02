use crate::settings::SETTINGS;

use actix_web::{HttpResponse, Responder};

pub async fn api_get_host_meta() -> impl Responder {
  HttpResponse::Ok()
    .content_type("application/xrd+xml; charset=utf-8")
    .body(format!(
      "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<XRD xmlns=\"http://docs.oasis-open.org/ns/xri/xrd-1.0\">
  <Link rel=\"lrdd\" template=\"{}/.well-known/webfinger?resource={{uri}}\"/>
</XRD>",
      SETTINGS.server.api_root_fqdn
    ))
}

#[cfg(test)]
mod tests {
  use actix_web::{http::header::ContentType, test, web, App};

  use super::*;

  #[actix_web::test]
  async fn test_index_get() {
    let app = test::init_service(App::new().route("/", web::get().to(api_get_host_meta))).await;
    let req = test::TestRequest::default()
      .insert_header(ContentType::json())
      .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
  }
}
