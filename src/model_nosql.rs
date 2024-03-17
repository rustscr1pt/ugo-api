use tokio::sync::MutexGuard;
use crate::data_models::AdminsData;



// Check if login data which was passed is correct and return bool statement
pub fn check_if_login_data_correct(login : String, password : String, pool : &mut MutexGuard<Vec<AdminsData>>) -> bool {
    for objects in pool.iter() {
        if objects.user_login == login && objects.user_password == password {
            return true
        }
    }
    return false
}