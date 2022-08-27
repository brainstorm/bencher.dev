use bencher_json::report::{
    data::{
        JsonReportAlert,
        JsonReportAlerts,
    },
    new::{
        JsonBenchmarks,
        JsonMetrics,
    },
    JsonLatency,
    JsonMetricsMap,
    JsonMinMaxAvg,
    JsonThroughput,
};
use chrono::offset::Utc;
use diesel::{
    expression_methods::BoolExpressionMethods,
    JoinOnDsl,
    NullableExpressionMethods,
    QueryDsl,
    RunQueryDsl,
    SqliteConnection,
};
use dropshot::HttpError;
use uuid::Uuid;

use super::alerts::Alerts;
use crate::{
    db::{
        model::{
            benchmark::{
                InsertBenchmark,
                QueryBenchmark,
            },
            perf::{
                latency::QueryLatency,
                min_max_avg::QueryMinMaxAvg,
                throughput::QueryThroughput,
                InsertPerf,
                QueryPerf,
            },
            threshold::{
                alert::InsertAlert,
                statistic::StatisticKind,
                PerfKind,
            },
        },
        schema,
        schema::statistic as statistic_table,
    },
    diesel::ExpressionMethods,
    util::http_error,
};

const PERF_ERROR: &str = "Failed to create perf statistic.";

pub struct Thresholds {
    pub latency:    Option<Threshold>,
    pub throughput: Option<Threshold>,
    pub compute:    Option<Threshold>,
    pub memory:     Option<Threshold>,
    pub storage:    Option<Threshold>,
}

impl Thresholds {
    pub fn new(
        conn: &SqliteConnection,
        branch_id: i32,
        testbed_id: i32,
        benchmarks: JsonBenchmarks,
    ) -> Self {
        let metrics_map = JsonMetricsMap::from(benchmarks);

        Self {
            latency:    Threshold::new(conn, branch_id, testbed_id, PerfKind::Latency).ok(),
            throughput: Threshold::new(conn, branch_id, testbed_id, PerfKind::Throughput).ok(),
            compute:    Threshold::new(conn, branch_id, testbed_id, PerfKind::Compute).ok(),
            memory:     Threshold::new(conn, branch_id, testbed_id, PerfKind::Memory).ok(),
            storage:    Threshold::new(conn, branch_id, testbed_id, PerfKind::Storage).ok(),
        }
    }
}

pub struct Threshold {
    pub id:        i32,
    pub statistic: Statistic,
}

pub struct Statistic {
    pub id:          i32,
    pub uuid:        String,
    pub test:        StatisticKind,
    pub sample_size: i64,
    pub window:      i64,
    pub left_side:   Option<f32>,
    pub right_side:  Option<f32>,
}

impl Threshold {
    pub fn new(
        conn: &SqliteConnection,
        branch_id: i32,
        testbed_id: i32,
        kind: PerfKind,
    ) -> Result<Self, HttpError> {
        schema::statistic::table
            .inner_join(
                schema::threshold::table
                    .on(schema::statistic::id.eq(schema::threshold::statistic_id)),
            )
            .filter(
                schema::threshold::branch_id
                    .eq(branch_id)
                    .and(schema::threshold::testbed_id.eq(testbed_id))
                    .and(schema::threshold::kind.eq(kind as i32)),
            )
            .select((
                schema::threshold::id,
                schema::statistic::id,
                schema::statistic::uuid,
                schema::statistic::test,
                schema::statistic::sample_size,
                schema::statistic::window,
                schema::statistic::left_side,
                schema::statistic::right_side,
            ))
            .first::<(
                i32,
                i32,
                String,
                i32,
                Option<i64>,
                Option<i64>,
                Option<f32>,
                Option<f32>,
            )>(conn)
            .map(
                |(
                    threshold_id,
                    statistic_id,
                    uuid,
                    test,
                    sample_size,
                    window,
                    left_side,
                    right_side,
                )|
                 -> Result<Self, HttpError> {
                    let statistic = Statistic {
                        id: statistic_id,
                        uuid,
                        test: test.try_into()?,
                        sample_size: unwrap_sample_size(sample_size),
                        window: unwrap_window(window),
                        left_side,
                        right_side,
                    };
                    Ok(Self {
                        id: threshold_id,
                        statistic,
                    })
                },
            )
            .map_err(|_| http_error!(PERF_ERROR))?
    }
}

fn unwrap_sample_size(sample_size: Option<i64>) -> i64 {
    sample_size.unwrap_or(i64::MAX)
}

fn unwrap_window(window: Option<i64>) -> i64 {
    window
        .map(|window| {
            let now = Utc::now().timestamp_nanos();
            now - window
        })
        .unwrap_or_default()
}
