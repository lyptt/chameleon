use super::object::Object;

pub trait ActivityConvertible {
  fn to_object(&self, actor: &str) -> Option<Object>;
}

pub trait ActivityActor {
  fn get_private_key(&self) -> &str;
  fn get_fediverse_uri(&self) -> &str;
}
