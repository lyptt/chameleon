use std::str::FromStr;

use uuid::Uuid;

use crate::{
  db::repositories::Repositories,
  federation::activitypub::{federate_ext, FederateExtAction, FederateExtDestActor, FederateExtDestActorRef},
  helpers::api::map_ext_err,
  logic::LogicErr,
};

pub async fn federate_activitypub(
  context: &Option<Vec<String>>,
  activitypub_federate_ext_action: &Option<FederateExtAction>,
  activitypub_federate_ext_dest_actor: &Option<FederateExtDestActorRef>,
  repositories: &Repositories,
) -> Result<(), LogicErr> {
  let user_id = match context {
    Some(context) => match context.len() {
      0 => return Err(LogicErr::MissingRecord),
      _ => Uuid::from_str(&context[0]).map_err(map_ext_err)?,
    },
    None => return Err(LogicErr::MissingRecord),
  };

  let actor = repositories.users.fetch_by_id(&user_id).await?;
  let action = match activitypub_federate_ext_action {
    Some(action) => action,
    None => return Err(LogicErr::MissingRecord),
  };
  let dest_actor = match activitypub_federate_ext_dest_actor {
    Some(actor) => actor,
    None => return Err(LogicErr::MissingRecord),
  };
  let dest_actor = match dest_actor {
    FederateExtDestActorRef::None => FederateExtDestActor::None,
    FederateExtDestActorRef::Person(id) => FederateExtDestActor::Person(repositories.users.fetch_by_id(id).await?),
    FederateExtDestActorRef::Group(id) => match repositories.orbits.fetch_orbit(id).await? {
      Some(orbit) => FederateExtDestActor::Group(orbit),
      None => return Err(LogicErr::MissingRecord),
    },
  };

  federate_ext(
    action.clone(),
    &actor,
    &dest_actor,
    &repositories.posts,
    &repositories.orbits,
  )
  .await
}
