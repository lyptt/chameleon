use handlebars::Handlebars;
use lazy_static::lazy_static;

// TODO: It would be better if we could eagerly evaluate all of these static members so that panics are
//       predictable, i.e. the server either panics on startup or never panics

lazy_static! {
  pub static ref HANDLEBARS: Handlebars<'static> = {
    let mut hb = Handlebars::new();
    hb.register_template_string(
      "oauth_authorize",
      include_str!("../../public/html/oauth-authorize.html"),
    )
    .unwrap();
    hb.register_template_string(
      "oauth_authorize_app_err",
      include_str!("../../public/html/oauth-authorize-app-err.html"),
    )
    .unwrap();
    hb
  };
}
