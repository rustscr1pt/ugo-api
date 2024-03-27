use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub const SESSION_DURATION : u16 = 900; // duration of login session in seconds

#[derive(Debug, Serialize)]
pub struct Message { // An easy answer to show a result of some action
    pub is_succeed: bool,
    pub message : String
}
#[derive(Debug, Serialize)]
pub struct LoginTryMessage { // An answer fo try to log in the system. If successful - user gets a token. Otherwise, token = Nil
    pub reply : String,
    pub token : String,
}
#[derive(Debug, Deserialize)]
pub struct WriteDataBody { // A data which passed in by user from the site.
    pub email : String,
    pub name : String,
    pub about_customer : String
}

pub struct WriteToBaseNewCustomer { // Represents the struct which is written inside mySQL about the customer
    pub id : u16,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub owner_notes : OwnerNotes,
    pub object_logs : ObjectLogs
}
#[derive(Debug, Serialize)]
pub struct OwnerNotes {
    pub notes : Vec<String>
}
#[derive(Debug, Serialize)]
pub struct ObjectLogs {
    pub logs : Vec<String>
}

#[derive(Debug)]
pub struct AdminsData { // Represents the admin which is added in the admins stack of mySQL
    pub id : u16,
    pub user_login : String,
    pub user_password : String
}

#[derive(Debug, Deserialize)]
pub struct LoginRequestData { // A body which arrives when the login request is made.
    pub login : String,
    pub password : String
}
#[derive(Debug, Clone)]
pub struct ActiveSessionsPool {
    pub session_id : String,
    pub countdown_secs : u16
}
#[derive(Debug, Deserialize)]
pub struct AgeStorageCheck { // A bode which arrives to check age
    pub token : String
}

pub enum CheckFieldsCase { // Error cases in input field.
    Ok,
    Email,
    Name,
    AboutCustomer
}