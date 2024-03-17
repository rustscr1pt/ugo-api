use std::fmt::Display;
use std::sync::Arc;
use mysql::{PooledConn};
use tokio::sync::Mutex;
use warp::http::Method;
use warp::{Rejection, Reply, reply};
use warp::reply::{json, Json};
use crate::data_models::{AdminsData, LoginRequestData, LoginTryMessage, Message, ObjectLogs, OwnerNotes, WriteDataBody, WriteToBaseNewCustomer};
use crate::model_nosql::check_if_login_data_correct;
use crate::sql_model::insert_customer_in_table;

type WebResult<T> = Result<T, Rejection>;

pub async fn refuse_connection(_ : Method) -> WebResult<impl Reply> { // Refuse connection if it doesn't match any filters
    Ok(reply::with_header(json(&Message { reply: "This request is forbidden, connection is dropped".to_string()}), "Access-Control-Allow-Origin", "*"))
}


// Standard reply to customer's request at the main page
fn reply_with_message<T>(message : T) -> WebResult<reply::WithHeader<Json>>
    where T : Display
{
    Ok(reply::with_header(json(&Message{reply : message.to_string()}), "Access-Control-Allow-Origin", "*"))
}

// Reply for a login attempt at the admin panel
fn login_try_reply_with_message<T>(message: T, token : T) -> WebResult<reply::WithHeader<Json>>
    where T : Display
{
    Ok(reply::with_header(json(&LoginTryMessage{ reply: message.to_string(), token: token.to_string() }),"Access-Control-Allow-Origin", "*"))
}

// Write a new customer request to the mySQL database
pub async fn handle_writing_task(body : WriteDataBody, pool : Arc<Mutex<PooledConn>>) -> WebResult<impl Reply> {
    let mut sample_to_write : Vec<WriteToBaseNewCustomer> = Vec::with_capacity(1);
    sample_to_write.push(WriteToBaseNewCustomer {
        id: 0,
        request_status: "БЕЗ ВНИМАНИЯ".to_string(),
        customer_name: body.name,
        customer_email: body.email,
        customer_self_description: body.about_customer,
        owner_notes: OwnerNotes { notes: vec![] },
        object_logs: ObjectLogs { logs: vec![] }
    });
    let mut unlocked = pool.lock().await;
    match insert_customer_in_table(&mut unlocked, sample_to_write) // Insert and get a response if it was successful or not.
    {
        Ok(_) => {reply_with_message("OK")}
        Err(e) => {reply_with_message(e)}
    }
}

// A function to handle user login in the admin panel. Answers with the LoginTryMessage struct. If the login was successful it would return a session token. Otherwise NIL
pub async fn handle_admin_login(body : LoginRequestData, pool : Arc<Mutex<PooledConn>>, admin_pool : Arc<Mutex<Vec<AdminsData>>>) -> WebResult<impl Reply> {
    let mut unlocked_admin_pool = admin_pool.lock().await;
    match check_if_login_data_correct(body.login, body.password, &mut unlocked_admin_pool) {
        true => {
            let mut unlocked = pool.lock().await;
        }
        false => {
            login_try_reply_with_message("Failed to login! Check your login & password", "Nil")
        }
    }
}