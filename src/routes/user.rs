use actix_easy_multipart::{tempfile::Tempfile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rsa::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::{
  cdn::cdn_store::Cdn,
  db::{session_repository::SessionPool, user_repository::UserPool, user_stats_repository::UserStatsPool},
  helpers::{
    auth::{query_auth, require_auth},
    core::{build_api_err, build_api_not_found},
    math::div_up,
  },
  logic::user::{get_user_by_handle, get_user_by_id},
  model::{
    response::{ListResponse, ObjectResponse},
    user_account_pub::UserAccountPub,
  },
  net::jwt::JwtContext,
};

#[derive(Debug, Deserialize)]
pub struct FollowersQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

#[derive(Deserialize, Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProfileUpdateProp {
  Replace(String),
  Erase,
}

#[derive(Deserialize)]
pub struct ProfileUpdateRequest {
  pub handle: Option<String>,
  pub intro_md: Option<ProfileUpdateProp>,
  pub email: Option<ProfileUpdateProp>,
  pub password: Option<String>,
  pub url_1: Option<ProfileUpdateProp>,
  pub url_2: Option<ProfileUpdateProp>,
  pub url_3: Option<ProfileUpdateProp>,
  pub url_4: Option<ProfileUpdateProp>,
  pub url_5: Option<ProfileUpdateProp>,
  pub url_1_title: Option<ProfileUpdateProp>,
  pub url_2_title: Option<ProfileUpdateProp>,
  pub url_3_title: Option<ProfileUpdateProp>,
  pub url_4_title: Option<ProfileUpdateProp>,
  pub url_5_title: Option<ProfileUpdateProp>,
}

#[derive(MultipartForm)]
pub struct ProfileAssetsUpload {
  #[multipart(rename = "images[]")]
  images: Vec<Tempfile>,
}

pub async fn api_get_profile(
  sessions: web::Data<SessionPool>,
  users: web::Data<UserPool>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match get_user_by_id(&props.uid, &users).await {
    Ok(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_update_profile(
  sessions: web::Data<SessionPool>,
  users: web::Data<UserPool>,
  req: web::Json<ProfileUpdateRequest>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let session = match require_auth(&jwt, &sessions).await {
    Ok(session) => session,
    Err(res) => return res,
  };

  let mut user = match users.fetch_by_id(&session.uid).await {
    Ok(user) => user,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  if let Some(handle) = &req.handle {
    user.handle = handle.to_owned();
  }

  if let Some(val) = &req.intro_md {
    let new_val = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };

    if let Some(new_val) = new_val {
      let intro_html = markdown::to_html(&new_val);
      user.intro_md = Some(new_val);
      user.intro_html = Some(intro_html);
    }
  }

  if let Some(val) = &req.email {
    user.email = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(password) = &req.password {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
      Ok(h) => h,
      Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
    }
    .to_string();
    user.password_hash = Some(password_hash);
  }

  if let Some(val) = &req.url_1 {
    user.url_1 = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_2 {
    user.url_2 = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_3 {
    user.url_3 = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_4 {
    user.url_4 = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_5 {
    user.url_5 = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_1_title {
    user.url_1_title = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_2_title {
    user.url_2_title = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_3_title {
    user.url_3_title = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_4_title {
    user.url_4_title = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  if let Some(val) = &req.url_5_title {
    user.url_5_title = match val {
      ProfileUpdateProp::Replace(val) => Some(val.to_owned()),
      ProfileUpdateProp::Erase => None,
    };
  }

  match users.update_from(&user).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_update_profile_assets(
  sessions: web::Data<SessionPool>,
  users: web::Data<UserPool>,
  cdn: web::Data<Cdn>,
  form: MultipartForm<ProfileAssetsUpload>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  if form.images.len() != 1 {
    return build_api_err(400, "Invalid image count".to_string(), None);
  }

  let session = match require_auth(&jwt, &sessions).await {
    Ok(session) => session,
    Err(res) => return res,
  };

  let mut user = match users.fetch_by_id(&session.uid).await {
    Ok(user) => user,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let avatar_content_type = match mime_guess::from_path(
    &form.images[0]
      .file_name
      .to_owned()
      .unwrap_or_else(|| ".§§§".to_string()),
  )
  .first()
  {
    Some(m) => m.to_string(),
    None => return build_api_err(500, "Unsupported file type".to_string(), None),
  };

  let avatar_uri = match {
    let file_name = format!("media/{}/or/{}", session.uid, Uuid::new_v4());

    let path = match cdn
      .upload_tmp_file(&form.images[0], &avatar_content_type, &file_name)
      .await
    {
      Ok(path) => path,
      Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
    };

    Some(format!("/{}", path))
  } {
    Some(uri) => uri,
    None => return build_api_err(500, "Image upload failed".to_string(), None),
  };

  user.avatar_url = Some(avatar_uri);

  match users.update_from(&user).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_get_user_profile(users: web::Data<UserPool>, handle: web::Path<String>) -> impl Responder {
  match get_user_by_handle(&handle, &users).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_stats(
  sessions: web::Data<SessionPool>,
  user_stats: web::Data<UserStatsPool>,
  jwt: web::ReqData<JwtContext>,
  handle: web::Path<String>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  match user_stats.fetch_for_user(&handle, &own_user_id).await {
    Some(user) => HttpResponse::Ok().json(ObjectResponse { data: user }),
    None => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_followers(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let user_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return build_api_not_found(handle.to_string()),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_followers_count(&user_id).await;

  match users.fetch_followers(&user_id, page_size, page * page_size).await {
    Ok(users) => HttpResponse::Ok().json(ListResponse {
      data: users.into_iter().map(UserAccountPub::from).collect(),
      page,
      total_items: users_count,
      total_pages: div_up(users_count, page_size) + 1,
    }),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_get_user_following(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let user_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return build_api_not_found(handle.to_string()),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_following_count(&user_id).await;

  match users.fetch_following(&user_id, page_size, page * page_size).await {
    Ok(users) => HttpResponse::Ok().json(ListResponse {
      data: users.into_iter().map(UserAccountPub::from).collect(),
      page,
      total_items: users_count,
      total_pages: div_up(users_count, page_size) + 1,
    }),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}
