use actix_easy_multipart::{tempfile::Tempfile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
  cdn::cdn_store::Cdn,
  db::{
    follow_repository::FollowPool, job_repository::JobPool, orbit_repository::OrbitPool,
    post_attachment_repository::PostAttachmentPool, post_repository::PostPool, session_repository::SessionPool,
    user_repository::UserPool,
  },
  helpers::{
    auth::{query_auth, require_auth},
    core::{build_api_err, build_api_not_found},
    math::div_up,
  },
  logic::post::{
    create_post, get_global_posts, get_global_posts_count, get_post, get_user_friends_posts,
    get_user_friends_posts_count, get_user_posts, get_user_posts_count, upload_post_files, CreatePostResult,
    NewPostRequest, NewPostResponse,
  },
  model::{
    access_type::AccessType,
    job::JobStatus,
    job::NewJob,
    queue_job::{QueueJob, QueueJobType},
    response::{JobResponse, ListResponse, ObjectResponse},
  },
  net::jwt::JwtContext,
  work_queue::queue::Queue,
};

#[derive(Debug, Deserialize)]
pub struct PostsQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

#[derive(MultipartForm)]
pub struct PostUpload {
  #[multipart(rename = "images[]")]
  images: Vec<Tempfile>,
}

pub async fn api_get_user_own_feed(
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  query: web::Query<PostsQuery>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  let user_id = props.uid;
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match get_user_posts_count(&user_id, &posts).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match get_user_posts(&user_id, page_size, page * page_size, &posts).await {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_get_user_friends_feed(
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  query: web::Query<PostsQuery>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  let user_id = props.uid;
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match get_user_friends_posts_count(&user_id, &posts).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match get_user_friends_posts(&user_id, page_size, page * page_size, &posts).await {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_get_post(
  users: web::Data<UserPool>,
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  follows: web::Data<FollowPool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let current_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => users.fetch_id_by_fediverse_id(&props.sub).await,
    None => None,
  };

  let post = match get_post(&post_id, &current_user_id, &posts).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return build_api_not_found(post_id.to_string()),
    },
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  if post.visibility == AccessType::PublicFederated
    || post.visibility == AccessType::PublicLocal
    || post.visibility == AccessType::Unlisted
  {
    return HttpResponse::Ok().json(ObjectResponse { data: post });
  }

  match current_user_id {
    Some(current_user_id) => {
      if post.user_id == current_user_id {
        return HttpResponse::Ok().json(ObjectResponse { data: post });
      }

      if post.visibility == AccessType::FollowersOnly
        && follows.user_follows_poster(&post.post_id, &current_user_id).await
      {
        return HttpResponse::Ok().json(ObjectResponse { data: post });
      }

      HttpResponse::NotFound().finish()
    }
    None => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_post(
  users: web::Data<UserPool>,
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  follows: web::Data<FollowPool>,
  ids: web::Path<(String, Uuid)>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  // We discard the user id since post ids are unique in our db
  let post_id = ids.1;

  let current_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => users.fetch_id_by_fediverse_id(&props.sub).await,
    None => None,
  };

  let post = match get_post(&post_id, &current_user_id, &posts).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return build_api_not_found(post_id.to_string()),
    },
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  if post.visibility == AccessType::PublicFederated
    || post.visibility == AccessType::PublicLocal
    || post.visibility == AccessType::Unlisted
  {
    return HttpResponse::Ok().json(ObjectResponse { data: post });
  }

  match current_user_id {
    Some(current_user_id) => {
      if post.user_id == current_user_id {
        return HttpResponse::Ok().json(ObjectResponse { data: post });
      }

      if post.visibility == AccessType::FollowersOnly
        && follows.user_follows_poster(&post.post_id, &current_user_id).await
      {
        return HttpResponse::Ok().json(ObjectResponse { data: post });
      }

      HttpResponse::NotFound().finish()
    }
    None => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_global_feed(posts: web::Data<PostPool>, query: web::Query<PostsQuery>) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match get_global_posts_count(&posts).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match get_global_posts(page_size, page * page_size, &posts).await {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_get_orbit_feed(
  posts: web::Data<PostPool>,
  orbits: web::Data<OrbitPool>,
  orbit_shortcode: web::Path<String>,
  query: web::Query<PostsQuery>,
) -> impl Responder {
  let orbit_id = match orbits.fetch_orbit_id_from_shortcode(&orbit_shortcode).await {
    Some(id) => id,
    None => return build_api_not_found(orbit_shortcode.to_string()),
  };

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

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_get_user_posts(
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  users: web::Data<UserPool>,
  query: web::Query<PostsQuery>,
  handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  let target_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return HttpResponse::NotFound().finish(),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_user_public_feed(&target_id, &user_id).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_user_public_feed(&target_id, &user_id, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_get_user_liked_posts(
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  users: web::Data<UserPool>,
  query: web::Query<PostsQuery>,
  handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  let target_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return HttpResponse::NotFound().finish(),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match posts.count_user_public_likes_feed(&target_id, &user_id).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match posts
    .fetch_user_public_likes_feed(&target_id, &user_id, page_size, page * page_size)
    .await
  {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_create_post(
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  req: web::Json<NewPostRequest>,
  jwt: web::ReqData<JwtContext>,
  queue: web::Data<Queue>,
  jobs: web::Data<JobPool>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_post(&posts, &jobs, &queue, &req, &props.uid).await {
    Ok(result) => match result {
      CreatePostResult::WaitingForImages(post_id) => HttpResponse::Ok().json(NewPostResponse { id: post_id }),
      CreatePostResult::JobQueued(job_id) => HttpResponse::Ok().json(JobResponse { job_id }),
    },
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_upload_post_image(
  form: MultipartForm<PostUpload>,
  post_id: web::Path<Uuid>,
  cdn: web::Data<Cdn>,
  queue: web::Data<Queue>,
  jwt: web::ReqData<JwtContext>,
  sessions: web::Data<SessionPool>,
  posts: web::Data<PostPool>,
  jobs: web::Data<JobPool>,
  post_attachments: web::Data<PostAttachmentPool>,
) -> impl Responder {
  if form.images.is_empty() {
    return HttpResponse::BadRequest().finish();
  }

  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match upload_post_files(
    &posts,
    &jobs,
    &post_attachments,
    &post_id,
    &props.uid,
    &cdn,
    &queue,
    &form.images,
  )
  .await
  {
    Ok(job_id) => HttpResponse::Ok().json(JobResponse { job_id }),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_boost_post(
  sessions: web::Data<SessionPool>,
  jobs: web::Data<JobPool>,
  queue: web::Data<Queue>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  let user_id = props.uid;

  let job_id = match jobs
    .create(NewJob {
      created_by_id: Some(user_id),
      status: JobStatus::NotStarted,
      record_id: Some(*post_id),
      associated_record_id: None,
    })
    .await
  {
    Ok(id) => id,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::CreateBoostEvents)
    .build();

  match queue.send_job(job).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_unboost_post(
  sessions: web::Data<SessionPool>,
  jobs: web::Data<JobPool>,
  queue: web::Data<Queue>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  let user_id = props.uid;

  let job_id = match jobs
    .create(NewJob {
      created_by_id: Some(user_id),
      status: JobStatus::NotStarted,
      record_id: Some(*post_id),
      associated_record_id: None,
    })
    .await
  {
    Ok(id) => id,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::DeleteBoostEvents)
    .build();

  match queue.send_job(job).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}
