use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BooleanOrShowObjectVec {
    Single(bool),
    Multiple(Vec<super::primitives::ShowInterval>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IntegerOrNumberObjectVec {
    Single(u32),
    Multiple(Vec<super::primitives::NumberInterval>),
}
