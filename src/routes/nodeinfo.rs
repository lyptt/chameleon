use crate::{
  db::{comment_repository::CommentPool, post_repository::PostPool, user_repository::UserPool},
  settings::SETTINGS,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use strum::{Display, EnumString};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoLink {
  rel: &'static str,
  href: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoResponse {
  links: Vec<NodeInfoLink>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoSoftware2_1 {
  name: &'static str,
  version: &'static str,
  repository: &'static str,
  homepage: &'static str,
}

#[derive(Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
enum NodeInfoProtocol2_1 {
  ActivityPub,
  BuddyCloud,
  Dfrn,
  Diaspora,
  Libertree,
  OStatus,
  PumpIO,
  Tent,
  Xmpp,
  Zot,
}

#[derive(Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
enum NodeInfoInboundService2_1 {
  #[strum(serialize = "atom1.0")]
  Atom1_0,
  GnuSocial,
  Imap,
  Pnut,
  Pop3,
  PumpIo,
  #[strum(serialize = "rss2.0")]
  Rss2_0,
  Twitter,
}

#[derive(Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
enum NodeInfoOutboundService2_1 {
  #[strum(serialize = "atom1.0")]
  Atom1_0,
  Blogger,
  BuddyCloud,
  Diaspora,
  DreamWidth,
  Drupal,
  Facebook,
  Friendica,
  GNUSocial,
  Google,
  InsaneJournal,
  Libertree,
  LinkedIn,
  LiveJournal,
  MediaGoblin,
  MySpace,
  Pinterest,
  Pnut,
  Posterous,
  PumpIO,
  RedMatrix,
  #[strum(serialize = "rss2.0")]
  Rss2_0,
  Smtp,
  Tent,
  Tumblr,
  Twitter,
  WordPress,
  Xmpp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoServices2_1 {
  inbound: Vec<NodeInfoInboundService2_1>,
  outbound: Vec<NodeInfoOutboundService2_1>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoUsers2_1 {
  total: i64,
  #[serde(rename = "activeHalfyear")]
  active_half_year: i64,
  active_month: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoUsage2_1 {
  users: NodeInfoUsers2_1,
  local_posts: i64,
  local_comments: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfoResponse2_1 {
  version: &'static str,
  software: NodeInfoSoftware2_1,
  protocols: Vec<NodeInfoProtocol2_1>,
  services: NodeInfoServices2_1,
  open_registrations: bool,
  usage: NodeInfoUsage2_1,
}

pub async fn api_get_nodeinfo() -> impl Responder {
  HttpResponse::Ok().json(NodeInfoResponse {
    links: vec![NodeInfoLink {
      rel: "http://nodeinfo.diaspora.software/ns/schema/2.1",
      href: format!("{}/nodeinfo/2.1", SETTINGS.server.api_fqdn),
    }],
  })
}

pub async fn api_get_nodeinfo_2_1(
  posts: web::Data<PostPool>,
  comments: web::Data<CommentPool>,
  users: web::Data<UserPool>,
) -> impl Responder {
  let post_count = posts.fetch_post_count().await;
  let comment_count = comments.fetch_comment_count().await;
  let user_count = users.fetch_user_count().await;

  HttpResponse::Ok().json(NodeInfoResponse2_1 {
    version: "2.1",
    software: NodeInfoSoftware2_1 {
      name: env!("CARGO_PKG_NAME"),
      version: env!("CARGO_PKG_VERSION"),
      repository: env!("CARGO_PKG_REPOSITORY"),
      homepage: env!("CARGO_PKG_HOMEPAGE"),
    },
    protocols: vec![NodeInfoProtocol2_1::ActivityPub],
    services: NodeInfoServices2_1 {
      inbound: vec![],
      outbound: vec![],
    },
    open_registrations: true,
    usage: NodeInfoUsage2_1 {
      users: NodeInfoUsers2_1 {
        total: user_count,
        active_half_year: user_count,
        active_month: user_count,
      },
      local_posts: post_count,
      local_comments: comment_count,
    },
  })
}
