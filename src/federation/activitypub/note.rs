use crate::{
  activitypub::{document::ActivityPubDocument, object::Object},
  logic::LogicErr,
  model::user::User,
};

pub async fn federate_create_note(doc: &ActivityPubDocument, obj: Object, actor: User) -> Result<(), LogicErr> {
  Ok(())
}
