use std::fs;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use mysql::{Error, params, Pool, PooledConn};
use mysql::prelude::Queryable;
use crate::data_models::{AdminsData, WriteToBaseNewCustomer};

// /Users/egorivanov/Desktop/mysql.txt
// C:\Users\User\Desktop\mysql.txt

pub fn establish_connection() -> PooledConn { // First action to check the connection and establish a working pool
    let pool = Pool::new(fs::read_to_string(r#"/Users/egorivanov/Desktop/mysql.txt"#).unwrap().trim()).expect("Couldn't connect to a base");
    println!("Connection with MySQL pool is established!");
    return pool.get_conn().unwrap();
}

pub async fn fill_the_admins(pool : Arc<Mutex<PooledConn>>) -> Result<Vec<AdminsData>, Error> { // Fill the admins pool from the mySQL database
    let mut unlocked = pool.lock().await;
    match unlocked.query_map("SELECT id, user_login, user_password FROM ugo_admin_accounts",
    |(id, user_login, user_password)| {
        AdminsData {
            id,
            user_login,
            user_password
        }
    }
    ) {
        Ok(result) => {
            println!("{:#?}", result);
            return Ok(result)
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(e)
        }
    }
}

// Insert a customer with its data to the MySQL table
pub fn insert_customer_in_table(unlocked : &mut MutexGuard<PooledConn>, sample_to_write : Vec<WriteToBaseNewCustomer>) -> mysql::Result<(), Error> {
    return unlocked.exec_batch(r"INSERT INTO ugo_customers_request VALUES (:id, :request_status, :customer_name, :customer_email, :customer_self_description, NOW(), :owner_notes, :object_logs)",
                               sample_to_write.iter().map(|value| params!{
        "id" => value.id,
        "request_status" => &value.request_status,
        "customer_name" => &value.customer_name,
        "customer_email" => &value.customer_email,
        "customer_self_description" => &value.customer_self_description,
        "owner_notes" => serde_json::to_string(&value.owner_notes).unwrap(),
        "object_logs" => serde_json::to_string(&value.object_logs).unwrap()
    }))
}