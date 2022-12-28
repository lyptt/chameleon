use tokio_postgres::Row;

use crate::logic::LogicErr;

pub trait FromRow
where
  Self: std::marker::Sized,
{
  fn from_row(row: Row) -> Option<Self>;
}

pub trait FromRowJoin
where
  Self: std::marker::Sized,
{
  fn from_row_join(row: &Row) -> Option<Self>;
}

pub trait FromRows
where
  Self: std::marker::Sized,
{
  fn from_rows(row: Vec<Row>) -> Result<Vec<Self>, LogicErr>;
}
