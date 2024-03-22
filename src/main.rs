use std::sync::Arc;
use mysql::prelude::WithParams;
use tokio::sync::Mutex;
use warp::Filter;
use crate::data_models::ActiveSessionsPool;
use crate::dep_injector::{with_admins_base, with_pool, with_session_pool};
use crate::model_nosql::{refresh_pool_connection, spawn_async_thread_cleaner};
use crate::sql_model::{establish_connection, fill_the_admins};
use crate::warp_handler::{handle_admin_login, handle_writing_task};

mod sql_model;
mod dep_injector;
mod cors_config;
mod warp_handler;
mod data_models;
mod model_nosql;

// /Users/egorivanov/Desktop/mysql.txt
// C:\Users\User\Desktop\mysql.txt
pub const FILE_LOCATION : &'static str = r#"mysql.txt"#;

#[tokio::main]
async fn main() {
    let arc_sql = Arc::new(Mutex::new(establish_connection()));

    let sessions_active_pool : Arc<Mutex<Vec<ActiveSessionsPool>>> = Arc::new(Mutex::new(Vec::new()));

    let check_base = Arc::new(Mutex::new(fill_the_admins(Arc::clone(&arc_sql)).await.unwrap())); // Fill the base with available admins from MySQL

    spawn_async_thread_cleaner(Arc::clone(&sessions_active_pool)); // spawn a cleaner of active sessions pool with a cool-down

    refresh_pool_connection(Arc::clone(&arc_sql)); // spawn a refresher for MySQL connection


    let write_route = warp::path!("data" / "write") // Write a data about customer to MySQL
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(Arc::clone(&arc_sql)))
        .and_then(handle_writing_task)
        .with(cors_config::get());

    let admin_login = warp::path!("admin" / "login") // Work with an attempt of logging in admin panel
        .and(warp::post())
        .and(warp::body::json())
        .and(with_session_pool(Arc::clone(&sessions_active_pool)))
        .and(with_admins_base(Arc::clone(&check_base)))
        .and_then(handle_admin_login)
        .with(cors_config::get());

    let refuse_connection = warp::any() // Refuse the connection if it doesn't match any filters
        .and(warp::method())
        .and_then(warp_handler::refuse_connection)
        .with(cors_config::get());

    let routes = write_route.or(admin_login).or(refuse_connection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
