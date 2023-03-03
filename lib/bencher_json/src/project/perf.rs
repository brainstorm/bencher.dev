use chrono::serde::ts_milliseconds_option::deserialize as from_milli_ts;
use chrono::{DateTime, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::ser::{self, SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};
use url::Url;
use uuid::Uuid;

use crate::urlencoded::{
    from_urlencoded, from_urlencoded_list, to_urlencoded, to_urlencoded_list, UrlEncodedError,
};
use crate::ResourceId;

use super::metric::JsonMetric;

const QUERY_KEYS: [&str; 6] = [
    "metric_kind",
    "branches",
    "testbeds",
    "benchmarks",
    "start_time",
    "end_time",
];

/// `JsonPerfQueryParams` is the actual query parameters accepted by the server.
/// All query parameter values are therefore scalar values.
/// Arrays are represented as comma separated lists.
/// Optional date times are simply stored as their millisecond representation.
/// `JsonPerfQueryParams` should always be converted into `JsonPerfQuery` for full type level validation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfQueryParams {
    pub metric_kind: String,
    pub branches: String,
    pub testbeds: String,
    pub benchmarks: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

/// `JsonPerfQuery` is the full, strongly typed version of `JsonPerfQueryParams`.
/// It should always be used to validate `JsonPerfQueryParams`.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfQuery {
    pub metric_kind: ResourceId,
    pub branches: Vec<Uuid>,
    pub testbeds: Vec<Uuid>,
    pub benchmarks: Vec<Uuid>,
    #[serde(deserialize_with = "from_milli_ts")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_milli_ts")]
    pub end_time: Option<DateTime<Utc>>,
}

impl TryFrom<JsonPerfQueryParams> for JsonPerfQuery {
    type Error = UrlEncodedError;

    fn try_from(query_params: JsonPerfQueryParams) -> Result<Self, Self::Error> {
        let JsonPerfQueryParams {
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
        } = query_params;

        let metric_kind = from_urlencoded(&metric_kind)?;

        let branches = from_urlencoded_list(&branches)?;
        let testbeds = from_urlencoded_list(&testbeds)?;
        let benchmarks = from_urlencoded_list(&benchmarks)?;

        let start_time = from_milli_ts(serde_json::json!(start_time))?;
        let end_time = from_milli_ts(serde_json::json!(end_time))?;

        Ok(Self {
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
        })
    }
}

impl Serialize for JsonPerfQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let urlencoded = self.urlencoded().map_err(ser::Error::custom)?;
        let mut state = serializer.serialize_struct("JsonPerfQuery", urlencoded.len())?;
        for (key, value) in urlencoded {
            state.serialize_field(key, &value)?;
        }
        state.end()
    }
}

impl JsonPerfQuery {
    pub fn to_url(&self, endpoint: &str, path: &str) -> Result<Url, UrlEncodedError> {
        let mut url = Url::parse(endpoint)?;
        url.set_path(path);
        url.set_query(Some(&self.to_query_string()?));
        Ok(url)
    }

    pub fn to_query_string(&self) -> Result<String, UrlEncodedError> {
        let urlencoded = self.urlencoded()?;
        serde_urlencoded::to_string(urlencoded).map_err(Into::into)
    }

    fn urlencoded(&self) -> Result<[(&'static str, Option<String>); 6], UrlEncodedError> {
        let JsonPerfQuery {
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
        } = self;

        let metric_kind = Some(to_urlencoded(metric_kind)?);

        let branches = Some(to_urlencoded_list(branches)?);
        let testbeds = Some(to_urlencoded_list(testbeds)?);
        let benchmarks = Some(to_urlencoded_list(benchmarks)?);

        let start_time = if let Some(start_time) = start_time {
            Some(to_urlencoded(&start_time.timestamp_millis())?)
        } else {
            None
        };
        let end_time = if let Some(end_time) = end_time {
            Some(to_urlencoded(&end_time.timestamp_millis())?)
        } else {
            None
        };

        QUERY_KEYS
            .into_iter()
            .zip(
                [
                    metric_kind,
                    branches,
                    testbeds,
                    benchmarks,
                    start_time,
                    end_time,
                ]
                .into_iter(),
            )
            .collect::<Vec<_>>()
            .try_into()
            .map_err(UrlEncodedError::Vec)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerf {
    pub metric_kind: Uuid,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub results: Vec<JsonPerfMetrics>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfMetrics {
    pub branch: Uuid,
    pub testbed: Uuid,
    pub benchmark: Uuid,
    pub metrics: Vec<JsonPerfMetric>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfMetric {
    pub uuid: Uuid,
    pub iteration: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub version_number: u32,
    pub version_hash: Option<String>,
    pub metric: JsonMetric,
}
