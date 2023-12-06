mod days;
use days::*;

use axum::{
    routing::{get, post},
    Router,
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(minus1::fake_error))
        .route("/1/*nums", get(day01::power_nums))
        .route("/4/strength", post(day04::calc_strength))
        .route("/4/contest", post(day04::contest))
        .route("/6", post(day06::elfs_on_shelf));

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
