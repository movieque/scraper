mod dataset;
mod message;
mod error;


pub use dataset::*;
pub use message::*;
pub use error::*;


pub type Result<T> = std::result::Result<T, Error>;