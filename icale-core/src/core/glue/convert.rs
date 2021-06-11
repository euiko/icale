// copyright to https://github.com/gluesql/gluesql-js

use serde_json::map::Map;
use serde_json::value::Value as Json;

use gluesql::{Payload, Row, Value};

pub fn convert_payload(payload: Payload) -> (String, Json) {
    match payload {
        Payload::Create => ("CREATE".to_owned(), Json::Null),
        Payload::Insert(num) => ("INSERT".to_owned(), Json::from(num)),
        Payload::Select { rows, .. } => (
            "SELECT".to_owned(),
            Json::Array(rows.into_iter().map(convert_row).collect()),
        ),
        Payload::Delete(num) => ("DELETE".to_owned(), Json::from(num)),
        Payload::Update(num) => ("UPDATE".to_owned(), Json::from(num)),
        Payload::DropTable => ("DROP".to_owned(), Json::Null),
        Payload::AlterTable => ("ALTER TABLE".to_owned(), Json::Null),
    }
}

fn convert_row(row: Row) -> Json {
    let Row(values) = row;

    Json::Array(values.into_iter().map(convert_value).collect())
}

fn convert_value(value: Value) -> Json {
    use Value::*;
    type IntervalType = gluesql::Interval;

    match value {
        Bool(v) => Json::Bool(v),
        I64(v)  => Json::from(v),
        F64(v)  => Json::from(v),
        Str(v)  => Json::String(v),
        Date(v) => Json::String(format!("{}", v.format("%Y-%m-%d"))),
        Timestamp(v) => Json::String(format!("{}", v.format("%Y-%m-%dT%H:%M:%S%.6fZ"))),
        Time(v) => Json::String(format!("{}", v.format("%H:%M:%S%.6f"))),
        Interval(v) => {
            match v {
                IntervalType::Month(m) => Json::String(format!("{} month", m)),
                IntervalType::Microsecond(micros) => Json::String(format!("{} seconds", micros / 1_000_000)),
            }
        },
        Null => Json::Null,
    }
}