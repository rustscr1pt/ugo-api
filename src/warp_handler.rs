use std::fmt::Display;
use std::sync::Arc;
use mysql::{PooledConn};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use warp::http::Method;
use warp::{Rejection, Reply, reply};
use warp::reply::{json, Json};
use crate::data_structs::{ActiveSessionsPool, AdminsData, AgeStorageCheck, CheckFieldsCase, LoginRequestData, LoginTryMessage, Message, ObjectLogs, OwnerNotes, SESSION_DURATION, WriteDataBody, WriteToBaseNewCustomer};
use crate::operational_logic::{check_before_sending, check_if_login_data_correct};
use crate::mysql_logic::insert_customer_in_table;

type WebResult<T> = Result<T, Rejection>;

pub async fn refuse_connection(_ : Method) -> WebResult<impl Reply> { // Refuse connection if it doesn't match any filters
    Ok(reply::with_header(json(&Message { is_succeed: false, message: "This request is forbidden, connection is dropped".to_string() }), "Access-Control-Allow-Origin", "http://ugo-vape.ru/"))
}


// Standard reply to customer's request at the main page
fn reply_with_message<T>(condition : bool, message : T) -> WebResult<reply::WithHeader<Json>>
    where T : Display
{
    Ok(reply::with_header(json(&Message{ is_succeed: condition, message: message.to_string() }), "Access-Control-Allow-Origin", "http://ugo-vape.ru/"))
}

// Reply for a login attempt at the admin panel
fn login_try_reply_with_message<T>(message: T, token : T) -> WebResult<reply::WithHeader<Json>>
    where T : Display
{
    Ok(reply::with_header(json(&LoginTryMessage{ reply: message.to_string(), token: token.to_string() }),"Access-Control-Allow-Origin", "*"))
}

// Write a new customer request to the mySQL database
pub async fn handle_writing_task(body : WriteDataBody, pool : Arc<Mutex<PooledConn>>) -> WebResult<impl Reply> {
    match check_before_sending(&body) {
        CheckFieldsCase::Ok => {
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
                Ok(_) => {reply_with_message(true, "Спасибо! Ваш запрос был отправлен! Мы ответим вам как можно скорее.")}
                Err(e) => {reply_with_message(false, e)}
            }
        }
        CheckFieldsCase::Email => {
            reply_with_message(false, "Проверьте на правильность поле email")
        }
        CheckFieldsCase::Name => {
            reply_with_message(false, "Поле 'имя' должно содержать больше 1 символа")
        }
        CheckFieldsCase::AboutCustomer => {
            reply_with_message(false, "Поле 'о вас' должно содержать больше 1 символа")
        }
    }
}
// Check if user's token in localstorage matches with current. If matches - it is redirected to the web-site. Otherwise, to the banner page
pub async fn handle_auth_check(body : AgeStorageCheck, token_pool : Arc<RwLock<String>>) -> WebResult<impl Reply> {
    let active_token = token_pool.read().await;
    if *active_token == body.token {
        return reply_with_message(true, "You have already been authorized");
    }
    return reply_with_message(false, "Please verify your age.");
}

// A function to handle user login in the admin panel. Answers with the LoginTryMessage struct. If the login was successful it would return a session token. Otherwise NIL
pub async fn handle_admin_login(body : LoginRequestData, session_pool : Arc<Mutex<Vec<ActiveSessionsPool>>>, admin_pool : Arc<Mutex<Vec<AdminsData>>>) -> WebResult<impl Reply> {
    let mut unlocked_admin_pool = admin_pool.lock().await;
    match check_if_login_data_correct(body.login, body.password, &mut unlocked_admin_pool) {
        true => {
            // Create UUID and place it in the pool of active sessions.
            let created_session = ActiveSessionsPool { // Created a session to add inside the pool
                session_id: String::from(Uuid::new_v4()),
                countdown_secs: SESSION_DURATION,
            };
            let mut unlocked = session_pool.lock().await;
            unlocked.push(created_session.clone());
            drop(unlocked);
            login_try_reply_with_message("You're successfully logged in.", &created_session.session_id)
        }
        false => {
            login_try_reply_with_message("Failed to login! Check your login & password", "Nil")
        }
    }
}