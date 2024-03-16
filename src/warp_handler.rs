use std::fmt::Display;
use std::sync::Arc;
use mysql::{params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::Mutex;
use warp::http::Method;
use warp::{Rejection, Reply, reply};
use warp::reply::{json, Json};
use crate::cors_config::{get_acao, get_value};
use crate::data_models::{Message, ObjectLogs, OwnerNotes, WriteDataBody, WriteToBaseNewCustomer};

type WebResult<T> = Result<T, Rejection>;

pub async fn refuse_connection(_ : Method) -> WebResult<impl Reply> { // Refuse connection if it doesn't match any filters
    Ok(reply::with_header(json(&Message { reply: "This request is forbidden, connection is dropped".to_string()}), "Access-Control-Allow-Origin", "*"))
}

fn reply_with_message<T>(message : T) -> WebResult<reply::WithHeader<Json>>
    where T : Display
{
    Ok(reply::with_header(json(&Message{reply : message.to_string()}), get_acao(), get_value()))
}
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
    match unlocked.exec_batch(r"INSERT INTO ugo_customers_request VALUES (:id, :request_status, :customer_name, :customer_email, :customer_self_description, NOW(), :owner_notes, :object_logs)",
    sample_to_write.iter().map(|value| params!{
        "id" => value.id,
        "request_status" => &value.request_status,
        "customer_name" => &value.customer_name,
        "customer_email" => &value.customer_email,
        "customer_self_description" => &value.customer_self_description,
        "owner_notes" => serde_json::to_string(&value.owner_notes).unwrap(),
        "object_logs" => serde_json::to_string(&value.object_logs).unwrap()
    }))
    {
        Ok(_) => {reply_with_message("Ваша заявка успешно отправлена.")}
        Err(e) => {reply_with_message(e)}
    }
}