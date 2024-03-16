use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Message {
    pub reply : String
}
#[derive(Debug, Deserialize)]
pub struct WriteDataBody {
    pub email : String,
    pub name : String,
    pub about_customer : String
}
#[derive(Debug, Serialize)]
pub struct OwnerNotes {
    pub notes : Vec<String>
}
#[derive(Debug, Serialize)]
pub struct ObjectLogs {
    pub logs : Vec<String>
}

pub struct WriteToBaseNewCustomer {
    pub id : u16,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub owner_notes : OwnerNotes,
    pub object_logs : ObjectLogs
}