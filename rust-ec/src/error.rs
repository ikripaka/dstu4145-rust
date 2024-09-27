use thiserror::Error;

pub type Result<T> = core::result::Result<T, BinaryECError>;

#[derive(Error, Debug)]
pub enum BinaryECError
{
  #[error("Incorrect params")]
  InvalidParams(String),
}
