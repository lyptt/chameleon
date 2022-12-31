use actix_web::{web, HttpResponse, Responder};
use phf::phf_map;

struct StaticContentData {
  pub content_type: &'static str,
  pub data: &'static [u8],
}

static STATIC_CONTENT: phf::Map<&'static str, &'static StaticContentData> = phf_map! {
  "styles/styles.css" => &StaticContentData {
    content_type: "text/css",
    data: include_bytes!("../../public/static/styles/styles.css")
  },
  "images/logo.svg" => &StaticContentData {
    content_type: "image/svg+xml",
    data: include_bytes!("../../public/static/images/logo.svg")
  },
  "images/space.png" => &StaticContentData {
    content_type: "image/png",
    data: include_bytes!("../../public/static/images/space.png")
  },
  "fonts/Ubuntu-Medium.woff" => &StaticContentData {
    content_type: "font/woff",
    data: include_bytes!("../../public/static/fonts/Ubuntu-Medium.woff")
  },
  "fonts/Ubuntu-Medium.woff2" => &StaticContentData {
    content_type: "font/woff2",
    data: include_bytes!("../../public/static/fonts/Ubuntu-Medium.woff2")
  },
  "fonts/Ubuntu-Regular.woff" => &StaticContentData {
    content_type: "font/woff",
    data: include_bytes!("../../public/static/fonts/Ubuntu-Regular.woff")
  },
  "fonts/Ubuntu-Regular.woff2" => &StaticContentData {
    content_type: "font/woff2",
    data: include_bytes!("../../public/static/fonts/Ubuntu-Regular.woff2")
  },
  "android-chrome-384x384.png" => &StaticContentData {
    content_type: "image/png",
    data: include_bytes!("../../public/static/android-chrome-384x384.png")
  },
  "apple-touch-icon.png" => &StaticContentData {
     content_type: "image/png",
    data: include_bytes!("../../public/static/apple-touch-icon.png")
  },
  "browserconfig.xml" => &StaticContentData {
    content_type: "text/xml",
    data: include_bytes!("../../public/static/browserconfig.xml")
  },
  "favicon-16x16.png" => &StaticContentData {
    content_type: "image/png",
    data: include_bytes!("../../public/static/favicon-16x16.png")
  },
  "favicon-32x32.png" => &StaticContentData {
    content_type: "image/png",
    data: include_bytes!("../../public/static/favicon-32x32.png")
  },
  "favicon.ico" => &StaticContentData {
    content_type: "image/vnd.microsoft.icon",
    data: include_bytes!("../../public/static/favicon.ico")
  },
  "mstile-150x150.png" => &StaticContentData {
    content_type: "image/png",
    data: include_bytes!("../../public/static/mstile-150x150.png")
  },
  "safari-pinned-tab.svg" => &StaticContentData {
    content_type: "image/xvg+xml",
    data: include_bytes!("../../public/static/safari-pinned-tab.svg")
  },
  "site.webmanifest" => &StaticContentData {

    content_type: "application/manifest+json",
    data: include_bytes!("../../public/static/site.webmanifest")
  },
};

pub async fn web_serve_static(path: web::Path<String>) -> impl Responder {
  if let Some(data) = STATIC_CONTENT.get(&path) {
    return HttpResponse::Ok().content_type(data.content_type).body(data.data);
  }

  HttpResponse::NotFound().finish()
}
