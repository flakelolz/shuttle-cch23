use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Value {
    Default(Vec<String>),
    Splitted(Vec<Vec<String>>),
}

pub async fn pagination(
    Query(pagination): Query<Pagination>,
    Json(payload): Json<Vec<String>>,
) -> Json<Value> {
    let start = if let Some(offset) = pagination.offset {
        if offset > payload.len() {
            payload.len()
        } else {
            offset
        }
    } else {
        0
    };

    let end = if let Some(limit) = pagination.limit {
        if start + limit > payload.len() {
            payload.len()
        } else {
            start + limit
        }
    } else {
        payload.len()
    };

    let names = payload[start..end].to_vec();

    if let Some(split) = pagination.split {
        let response = names
            .chunks(split)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<String>>>();
        return Json(Value::Splitted(response));
    }

    let response = names.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    Json(Value::Default(response))
}
