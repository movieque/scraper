mod dataset;
mod message;
mod error;


pub use dataset::*;
pub use message::*;
pub use error::*;


pub type Result<T> = std::result::Result<T, Error>;


#[derive(serde::Deserialize)]
pub struct Object {
    pub id: u32,
}