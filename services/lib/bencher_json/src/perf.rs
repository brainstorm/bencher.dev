use chrono::{DateTime, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::report::{JsonLatency, JsonResource, JsonThroughput};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfQuery {
    pub branches: Vec<Uuid>,
    pub testbeds: Vec<Uuid>,
    pub benchmarks: Vec<Uuid>,
    pub kind: JsonPerfKind,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum JsonPerfKind {
    Latency,
    Throughput,
    Compute,
    Memory,
    Storage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerf {
    pub kind: JsonPerfKind,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub benchmarks: Vec<JsonPerfData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfData {
    pub branch: Uuid,
    pub testbed: Uuid,
    pub benchmark: Uuid,
    pub data: Vec<JsonPerfDatum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfDatum {
    pub uuid: Uuid,
    pub iteration: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub version_number: u32,
    pub version_hash: Option<String>,
    pub metrics: JsonPerfDatumKind,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum JsonPerfDatumKind {
    Latency(JsonLatency),
    Throughput(JsonThroughput),
    Compute(JsonResource),
    Memory(JsonResource),
    Storage(JsonResource),
}
