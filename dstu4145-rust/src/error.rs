use signature::Error;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Dstu4145Error>;

#[derive(Error, Debug)]
pub enum Dstu4145Error
{
  #[error("Incorrect params, error: '{0}'.")]
  InvalidParams(String),
  #[error("Invalid param length, desired: {0}, got: {1}, param name: '{2}'.")]
  InvalidParamLength(u64, u64, String),
  #[error("Got signature error: {0}.")]
  GotSignatureError(#[from] signature::Error),
  #[error("Got point in infinity on verifying signature, please check validity of the signature.")]
  GotPointInInfinity,
  #[error("Signature check failed, unequal check values. Please check validity of the keys.")]
  IncorrectSignature,
  #[error("Failed to parse hex string, check validity of it.")]
  ParseBigIntError(#[from] num_bigint::ParseBigIntError),
}

impl Into<signature::Error> for Dstu4145Error
{
  fn into(self) -> Error { Error::from_source(self) }
}
