use crate::{
  activitypub::{document::ActivityPubDocument, object::Object},
  logic::LogicErr,
  model::user::User,
};

#[cfg(test)]
use mockall::automock;

pub struct FederationActivityPub {}

#[cfg_attr(test, automock)]
impl FederationActivityPub {
  async fn federate(doc: ActivityPubDocument, from_server_endpoint: &str) -> Result<(), LogicErr> {
    todo!();
  }

  async fn send_user_profile(user: &User, server_endpoint: &str) -> Result<(), LogicErr> {
    Err(LogicErr::MissingRecord)
  }

  async fn receive_user_profile<'a>(entity: Object, from_server_endpoint: &str) -> Result<(), LogicErr> {
    Err(LogicErr::MissingRecord)
  }
}
