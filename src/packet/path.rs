use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PathLeadTrailInterval {
    pub interval: String,
    pub epoch: String,
    pub number: [f64; 4],
}
