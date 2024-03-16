use std::sync::Arc;
use mysql::prelude::WithParams;
use tokio::sync::Mutex;
use warp::Filter;
use crate::dep_injector::with_pool;
use crate::sql_model::establish_connection;
use crate::warp_handler::handle_writing_task;

mod sql_model;
mod dep_injector;
mod cors_config;
mod warp_handler;
mod data_models;

#[tokio::main]
async fn main() {
    let arc_sql = Arc::new(Mutex::new(establish_connection()));

    let write_route = warp::path!("data" / "write")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(Arc::clone(&arc_sql)))
        .and_then(handle_writing_task)
        .with(cors_config::get());

    let routes = write_route;

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
