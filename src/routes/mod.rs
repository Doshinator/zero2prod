//! src/routes/mod.rs
mod health_check;
mod subscriptions;
mod greeting;
mod hello_world;
mod name;

pub use health_check::*; 
pub use subscriptions::*;
pub use greeting::*;
pub use hello_world::*;
pub use name::*;