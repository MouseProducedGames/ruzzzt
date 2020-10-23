use serde::{Deserialize, Serialize};

mod de;
mod error;
mod ser;

pub use crate::error::{Error, Result};
pub use crate::ser::{to_string, Serializer};
pub use de::{from_str, Deserializer};

#[derive(Clone, Deserialize, Serialize)]
pub enum ZValue {
    Boolean(bool, Vec<ZValue>),
    F32(f32, Vec<ZValue>),
    I32(i32, Vec<ZValue>),
    String(String, Vec<ZValue>),
    Null(Vec<ZValue>),
}
