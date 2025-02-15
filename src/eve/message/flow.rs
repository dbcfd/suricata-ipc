use crate::eve::message::date_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlowInfo {
    pub age: usize,
    pub pkts_toserver: usize,
    pub pkts_toclient: usize,
    pub bytes_toserver: usize,
    pub bytes_toclient: usize,
    #[serde(with = "date_format")]
    pub start: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub end: DateTime<Utc>,
    pub alerted: bool,
    pub state: super::State,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flow {
    #[serde(flatten)]
    pub event_fields: super::EventFields,
    #[serde(rename = "flow")]
    pub info: FlowInfo,
}
