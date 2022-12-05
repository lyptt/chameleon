use super::object::Object;

pub trait ActivityConvertible {
  fn to_object(&self, actor: &str) -> Object;
}
