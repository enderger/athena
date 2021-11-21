use thiserror::Error;

pub mod models;
pub mod serde;

#[derive(Error, Debug)]
pub enum AthenaError {}
