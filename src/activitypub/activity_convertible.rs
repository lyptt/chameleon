use super::activity::Activity;

pub trait ActivityConvertible<T> {
  fn to_activity(&self, base_uri: &str, actor_uri: &str) -> Option<Activity<T>>;
}
