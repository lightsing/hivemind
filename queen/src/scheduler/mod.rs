#[cfg(feature = "aws")]
pub mod aws;

pub trait Scheduler: Sized {}
