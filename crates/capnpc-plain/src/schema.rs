mod bootstrap;
#[rustfmt::skip]
mod schema_capnp;

pub use bootstrap::*;
pub use schema_capnp::{Field__Slot, Type, Value};
