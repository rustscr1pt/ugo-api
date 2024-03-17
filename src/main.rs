use std::sync::Arc;
use futures::SinkExt;
use mysql::prelude::WithParams;
use tokio::sync::Mutex;
use warp::Filter;
use crate::dep_injector::{with_admins_base, with_pool};
use crate::sql_model::{establish_connection, fill_the_admins};
use crate::warp_handler::handle_writing_task;

mod sql_model;
mod dep_injector;
mod cors_config;
mod warp_handler;
mod data_models;
mod model_nosql;

#[tokio::main]
async fn main() {
    let arc_sql = Arc::new(Mutex::new(establish_connection()));

    let check_base = Arc::new(Mutex::new(fill_the_admins(Arc::clone(&arc_sql)).await.unwrap())); // Fill the base with available admins from MySQL


    let write_route = warp::path!("data" / "write") // Write a data about customer to MySQL
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(Arc::clone(&arc_sql)))
        .and_then(handle_writing_task)
        .with(cors_config::get());

    let admin_login = warp::path!("admin" / "login") // Work with an attempt of logging in admin panel
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(Arc::clone(&arc_sql)))
        .and(with_admins_base(Arc::clone(&check_base)))
        .and_then()
        .with(cors_config::get());

    let refuse_connection = warp::any() // Refuse the connection if it doesn't match any filters
        .and(warp::method())
        .and_then(warp_handler::refuse_connection)
        .with(cors_config::get());

    let routes = write_route.or(admin_login).or(refuse_connection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
