use thiserror::Error;

pub mod models;
mod serde;

#[derive(Error, Debug)]
pub enum AthenaError {}
