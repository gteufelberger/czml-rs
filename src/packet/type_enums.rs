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
pub enum FloatOrNumberObjectVec {
    Single(f64),
    Multiple(Vec<super::primitives::NumberInterval>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColorOrColorInterval {
    Single(super::primitives::Color),
    Multiple(Vec<super::primitives::ColorInterval>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NumberOrPathLeadTrailInterval {
    Single(u32),
    Intervals(Vec<super::path::PathLeadTrailInterval>),
}
