use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)] // <-- this is a container attribute
pub struct S {
    #[serde(default)] // <-- this is a field attribute
    f: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "e")] // <-- this is also a container attribute
pub enum E {
    #[serde(rename = "a")] // <-- this is a variant attribute
    A(String),
}
