use std::sync::Arc;
use futures::SinkExt;
use mysql::prelude::WithParams;
use tokio::sync::{Mutex, RwLock};
use warp::Filter;
use crate::data_structs::ActiveSessionsPool;
use crate::dependency_injector::{with_admins_base, with_auth_token, with_pool, with_session_pool};
use crate::operational_logic::{refresh_pool_connection, set_token_and_refresh, spawn_async_thread_cleaner};
use crate::mysql_logic::{establish_connection, fill_the_admins};
use crate::warp_handler::{handle_admin_login, handle_auth_check, handle_writing_task};

mod mysql_logic;
mod dependency_injector;
mod cors_config_builder;
mod warp_handler;
mod data_structs;
mod operational_logic;

// /Users/egorivanov/Desktop/mysql.txt - MacOS
// C:\Users\User\Desktop\mysql.txt - Windows
// mysql.txt - Linux
pub const FILE_LOCATION : &'static str = r#"mysql.txt"#;

#[tokio::main]
async fn main() {
    let arc_sql = Arc::new(Mutex::new(establish_connection()));


    // let rw_token = Arc::new(RwLock::new(String::new())); // Daily generated token for passing an age verification

    // let sessions_active_pool : Arc<Mutex<Vec<ActiveSessionsPool>>> = Arc::new(Mutex::new(Vec::new()));
    //
    // let check_base = Arc::new(Mutex::new(fill_the_admins(Arc::clone(&arc_sql)).await.unwrap())); // Fill the base with available admins from MySQL
    //
    // spawn_async_thread_cleaner(Arc::clone(&sessions_active_pool)); // spawn a cleaner of active sessions pool with a cool-down

    refresh_pool_connection(Arc::clone(&arc_sql)); // spawn a refresher for MySQL connection

    // set_token_and_refresh(Arc::clone(&rw_token)); // spawn a daily token creator for auth in age

    let write_route = warp::path!("data" / "write") // Write a data about customer to MySQL
        .and(warp::post())
        .and(warp::body::content_length_limit(4096)) // restrict data size of json to 4kb
        .and(warp::body::json())
        .and(with_pool(Arc::clone(&arc_sql)))
        .and_then(handle_writing_task)
        .with(cors_config_builder::get());

    // Not needed at the moment!
    // let check_token = warp::path!("token" / "check") // Check the token inside the localstorage of browser to decide if user should pass age verification again
    //     .and(warp::post())
    //     .and(warp::body::content_length_limit(4096))
    //     .and(warp::body::json())
    //     .and(with_auth_token(Arc::clone(&rw_token)))
    //     .and_then(handle_auth_check)
    //     .with(cors_config_builder::get());

    // In progress, release when the panel is ready.
    // let admin_login = warp::path!("admin" / "login") // Work with an attempt of logging in admin panel
    //     .and(warp::post())
    //     .and(warp::body::json())
    //     .and(with_session_pool(Arc::clone(&sessions_active_pool)))
    //     .and(with_admins_base(Arc::clone(&check_base)))
    //     .and_then(handle_admin_login)
    //     .with(cors_config::get());

    let refuse_connection = warp::any() // Refuse the connection if it doesn't match any filters
        .and(warp::method())
        .and_then(warp_handler::refuse_connection)
        .with(cors_config_builder::get());

    let routes = write_route.or(refuse_connection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
