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
    comment_repository::CommentPool, follow_repository::FollowPool, job_repository::JobPool,
    orbit_repository::OrbitPool, post_repository::PostPool, session_repository::SessionPool,
    user_orbit_repository::UserOrbitPool, user_repository::UserPool,
  },
  helpers::{
    auth::query_auth,
    core::{build_api_err, build_api_not_found, map_api_err},
    types::ACTIVITY_JSON_CONTENT_TYPE,
  },
  logic::{
    comment::{activitypub_get_comment, activitypub_get_comments},
    post::get_post,
    user::get_user_by_id,
  },
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  net::{http_sig::build_origin_data, jwt::JwtContext},
  settings::SETTINGS,
  work_queue::queue::Queue,
};

use super::{comment::CommentsQuery, orbit::MembersQuery, post::PostsQuery, user::FollowersQuery};

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

  let post_obj = match post.to_object(&format!("{}/user/{}", SETTINGS.server.api_fqdn, post.user_id)) {
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
  query: web::Query<PostsQuery>,
  user_id: web::Path<Uuid>,
) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_user_public_feed(&user_id, &None).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_user_public_feed(&user_id, &None, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let doc = create_activitypub_ordered_collection_page_feed(
    &format!("{}/user/{}/feed", SETTINGS.server.api_fqdn, user_id),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    posts_count.try_into().unwrap_or_default(),
    posts,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

pub async fn api_activitypub_get_federated_orbit_posts(
  posts: web::Data<PostPool>,
  query: web::Query<PostsQuery>,
  orbit_id: web::Path<Uuid>,
) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_global_federated_orbit_feed(&orbit_id).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_global_federated_orbit_feed(&orbit_id, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let doc = create_activitypub_ordered_collection_page_feed(
    &format!("{}/orbit/{}/feed", SETTINGS.server.api_fqdn, orbit_id),
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
  query: web::Query<PostsQuery>,
  user_id: web::Path<Uuid>,
) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_user_public_likes_feed(&user_id, &None).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_user_public_likes_feed(&user_id, &None, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let doc = create_activitypub_ordered_collection_page_specific_feed(
    &format!("{}/user/{}/likes", SETTINGS.server.api_fqdn, user_id),
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
  user_id: web::Path<Uuid>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_followers_count(&user_id).await;

  let users = match users.fetch_followers(&user_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/user/{}/followers", SETTINGS.server.api_fqdn, user_id),
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
  user_id: web::Path<Uuid>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_following_count(&user_id).await;

  let users = match users.fetch_following(&user_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/user/{}/following", SETTINGS.server.api_fqdn, user_id),
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

pub async fn api_activitypub_get_user_profile(users: web::Data<UserPool>, user_id: web::Path<Uuid>) -> impl Responder {
  match get_user_by_id(&user_id, &users).await {
    Ok(user) => match user.to_object("") {
      Some(obj) => {
        let doc = ActivityPubDocument::new(obj);
        HttpResponse::Ok()
          .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
          .json(doc)
      }
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_activitypub_get_orbit(orbits: web::Data<OrbitPool>, orbit_id: web::Path<Uuid>) -> impl Responder {
  match orbits.fetch_orbit(&orbit_id).await {
    Ok(orbit) => match orbit {
      Some(orbit) => match orbit.to_object("") {
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

pub async fn api_activitypub_get_orbit_members(
  user_orbits: web::Data<UserOrbitPool>,
  orbit_id: web::Path<Uuid>,
  query: web::Query<MembersQuery>,
) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = match user_orbits.count_users(&orbit_id).await {
    Ok(count) => count,
    Err(_) => return HttpResponse::NotFound().finish(),
  };

  let users = match user_orbits.fetch_users(&orbit_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/orbit/{}/members", SETTINGS.server.api_fqdn, orbit_id),
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

pub async fn api_activitypub_get_comment(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  posts: web::Data<PostPool>,
  ids: web::Path<(Uuid, Uuid)>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  match activitypub_get_comment(&comments, &posts, &ids.0, &ids.1, &own_user_id).await {
    Ok(response) => HttpResponse::Ok().json(response),
    Err(err) => map_api_err(err),
  }
}

pub async fn api_activitypub_get_comments(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  posts: web::Data<PostPool>,
  query: web::Query<CommentsQuery>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  match activitypub_get_comments(&posts, &comments, &post_id, &own_user_id, &query.page, &query.page_size).await {
    Ok(response) => HttpResponse::Ok().json(response),
    Err(err) => map_api_err(err),
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
  user_id: web::Path<Uuid>,
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
    .context(vec![user_id.to_string()])
    .origin_data(origin_data)
    .build();

  match queue.send_job(job).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_activitypub_federate_orbit_inbox(
  req: HttpRequest,
  orbit_id: web::Path<Uuid>,
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
    .context(vec![orbit_id.to_string()])
    .origin_data(origin_data)
    .build();

  match queue.send_job(job).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}
