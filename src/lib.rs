use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum ZValue {
    Boolean(bool, Vec<ZValue>),
    F32(f32, Vec<ZValue>),
    I32(i32, Vec<ZValue>),
    String(String, Vec<ZValue>),
    Null(Vec<ZValue>),
}
