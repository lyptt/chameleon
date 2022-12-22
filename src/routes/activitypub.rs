use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible,
    activity_type::ActivityType,
    document::ActivityPubDocument,
    helpers::{
      create_activitypub_ordered_collection_page, create_activitypub_ordered_collection_page_feed,
      create_activitypub_ordered_collection_page_specific_feed,
    },
  },
  db::{
    follow_repository::FollowPool, job_repository::JobPool, post_repository::PostPool, session_repository::SessionPool,
    user_repository::UserPool,
  },
  helpers::{
    auth::query_auth,
    core::{build_api_err, build_api_not_found},
    types::ACTIVITY_JSON_CONTENT_TYPE,
  },
  logic::{post::get_post, user::get_user_by_handle},
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  net::{http_sig::build_origin_data, jwt::JwtContext},
  settings::SETTINGS,
  work_queue::queue::Queue,
};

use super::{post::PostsQuery, user::FollowersQuery};

pub async fn api_activitypub_get_post(
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  follows: web::Data<FollowPool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let user_props = query_auth(&jwt, &sessions).await;
  let current_user_id = match user_props {
    Some(p) => Some(p.uid),
    None => None,
  };

  let post = match get_post(&post_id, &current_user_id, &posts).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return build_api_not_found(post_id.to_string()),
    },
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let post_obj = match post.to_object(&format!("{}/users/{}", SETTINGS.server.api_fqdn, post.user_handle)) {
    Some(post) => post,
    None => return build_api_not_found(post_id.to_string()),
  };

  let doc = ActivityPubDocument::new(post_obj);

  if post.visibility == AccessType::PublicFederated
    || post.visibility == AccessType::PublicLocal
    || post.visibility == AccessType::Unlisted
  {
    return HttpResponse::Ok().json(doc);
  }

  match current_user_id {
    Some(current_user_id) => {
      if post.user_id == current_user_id {
        return HttpResponse::Ok().json(doc);
      }

      if post.visibility == AccessType::FollowersOnly
        && follows.user_follows_poster(&post.post_id, &current_user_id).await
      {
        return HttpResponse::Ok().json(doc);
      }

      HttpResponse::NotFound().finish()
    }
    None => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_activitypub_get_federated_user_posts(
  posts: web::Data<PostPool>,
  users: web::Data<UserPool>,
  query: web::Query<PostsQuery>,
  handle: web::Path<String>,
) -> impl Responder {
  let target_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return HttpResponse::NotFound().finish(),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_user_public_feed(&target_id, &None).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_user_public_feed(&target_id, &None, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let doc = create_activitypub_ordered_collection_page_feed(
    &format!("{}/users/{}/feed", SETTINGS.server.api_fqdn, handle),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    posts_count.try_into().unwrap_or_default(),
    posts,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

pub async fn api_activitypub_get_federated_user_liked_posts(
  posts: web::Data<PostPool>,
  users: web::Data<UserPool>,
  query: web::Query<PostsQuery>,
  handle: web::Path<String>,
) -> impl Responder {
  let target_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return HttpResponse::NotFound().finish(),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_user_public_likes_feed(&target_id, &None).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_user_public_likes_feed(&target_id, &None, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let doc = create_activitypub_ordered_collection_page_specific_feed(
    &format!("{}/users/{}/likes", SETTINGS.server.api_fqdn, handle),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    posts_count.try_into().unwrap_or_default(),
    posts,
    ActivityType::Like,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

pub async fn api_activitypub_get_user_followers(
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

  let users = match users.fetch_followers(&user_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/users/{}/followers", SETTINGS.server.api_fqdn, handle),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    users_count.try_into().unwrap_or_default(),
    users,
    None,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

pub async fn api_activitypub_get_user_following(
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

  let users = match users.fetch_following(&user_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/users/{}/following", SETTINGS.server.api_fqdn, handle),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    users_count.try_into().unwrap_or_default(),
    users,
    None,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

pub async fn api_activitypub_get_user_profile(users: web::Data<UserPool>, handle: web::Path<String>) -> impl Responder {
  match get_user_by_handle(&handle, &users).await {
    Ok(user) => match user {
      Some(user) => match user.to_object("") {
        Some(obj) => {
          let doc = ActivityPubDocument::new(obj);
          HttpResponse::Ok()
            .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
            .json(doc)
        }
        None => HttpResponse::NotFound().finish(),
      },
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_activitypub_federate_shared_inbox(
  req: HttpRequest,
  jobs: web::Data<JobPool>,
  data: web::Json<serde_json::Value>,
  queue: web::Data<Queue>,
) -> impl Responder {
  let origin_data = build_origin_data(&req);

  if SETTINGS.app.secure && origin_data.is_none() {
    return build_api_err(401, "signature".to_string(), None);
  }

  let job_id = match jobs
    .create(NewJob {
      created_by_id: None,
      status: JobStatus::NotStarted,
      record_id: None,
      associated_record_id: None,
    })
    .await
  {
    Ok(id) => id,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::FederateActivityPub)
    .data((*data).to_owned())
    .origin(SETTINGS.server.api_root_fqdn.to_owned())
    .origin_data(origin_data)
    .build();

  let json = serde_json::to_string_pretty(&job).unwrap();
  println!("{}", json);

  match queue.send_job(job).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_activitypub_federate_user_inbox(
  req: HttpRequest,
  user_handle: web::Path<String>,
  data: web::Json<serde_json::Value>,
  jobs: web::Data<JobPool>,
  queue: web::Data<Queue>,
) -> impl Responder {
  let origin_data = build_origin_data(&req);

  if SETTINGS.app.secure && origin_data.is_none() {
    return build_api_err(401, "signature".to_string(), None);
  }

  let job_id = match jobs
    .create(NewJob {
      created_by_id: None,
      status: JobStatus::NotStarted,
      record_id: None,
      associated_record_id: None,
    })
    .await
  {
    Ok(id) => id,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::FederateActivityPub)
    .data((*data).to_owned())
    .origin(req.connection_info().host().to_string())
    .context(vec![(*user_handle).clone()])
    .origin_data(origin_data)
    .build();

  match queue.send_job(job).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}
