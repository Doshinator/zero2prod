//! src/routes/mod.rs
#![allow(unused_variables)]
#![allow(warnings)]
mod greeting;
mod health_check;
mod hello_world;
mod name;
mod subscriptions;

pub use greeting::*;
pub use health_check::*;
pub use hello_world::*;
pub use name::*;
pub use subscriptions::*;
