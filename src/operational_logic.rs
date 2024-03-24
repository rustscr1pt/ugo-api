use std::fs;
use std::sync::Arc;
use std::time::Duration;
use mysql::{Pool, PooledConn};
use tokio::sync::{Mutex, MutexGuard, RwLock};
use tokio::time::sleep;
use crate::data_structs::{ActiveSessionsPool, AdminsData};
use crate::FILE_LOCATION;

pub fn refresh_pool_connection(to_refresh : Arc<Mutex<PooledConn>>) -> () {
    tokio::spawn(async move {
        let mut timer : u8 = 60;
        loop {
            if timer == 0 {
                let pool = Pool::new(fs::read_to_string(FILE_LOCATION).unwrap().trim()).expect("Couldn't connect to a base").get_conn().unwrap();
                let mut unlocked = to_refresh.lock().await;
                *unlocked = pool;
                drop(unlocked);
                println!("Connection with MySQL pool is refreshed");
                timer = 60;
            }
            else {
                sleep(Duration::from_secs(1)).await;
                timer -= 1;
                println!("{} seconds estimated till MySQL pool is refreshed.", timer);
            }
        }
    });
}

// Set age token for 24 hours and if user has the same allow it to web-site without age verification
pub fn set_token_and_refresh(token_holder : Arc<RwLock<String>>) -> () {
    tokio::spawn(async move {
        let mut token_set = token_holder.write().await;
        *token_set = String::from(uuid::Uuid::new_v4());
        println!("Daily token : {}", token_set);
        drop(token_set);
        let mut timer = 24u8;
        loop {
            if timer == 0 {
                let mut token_set = token_holder.write().await;
                *token_set = String::from(uuid::Uuid::new_v4());
                drop(token_set);
                println!("Daily token for enter has been refreshed!");
            }
            else {
                sleep(Duration::from_secs(3600)).await; // 3600 seconds == one hour
                timer -= 1;
                println!("{} hours left till token is refreshed", timer);
            }
        }
    });
}


// Check if login data which was passed is correct and return bool statement
pub fn check_if_login_data_correct(login : String, password : String, pool : &mut MutexGuard<Vec<AdminsData>>) -> bool {
    for objects in pool.iter() {
        if objects.user_login == login && objects.user_password == password {
            return true
        }
    }
    return false
}


// In progress, not needed at the moment.
pub fn spawn_async_thread_cleaner(active_sessions_pool: Arc<Mutex<Vec<ActiveSessionsPool>>>) -> () {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;
            let cloned = Arc::clone(&active_sessions_pool);
            let mut unlocked = cloned.lock().await;
            println!("{:#?}", unlocked);
            let rewrite = session_reduce_timer_and_filter(unlocked.clone());
            *unlocked = rewrite;
        }
    });
}

pub fn session_reduce_timer_and_filter(cloned : Vec<ActiveSessionsPool>) -> Vec<ActiveSessionsPool> {
    return cloned
        .iter()
        .map(|session| reduce_by_30(session))
        .collect::<Vec<ActiveSessionsPool>>()
        .into_iter()
        .filter(|session| session.countdown_secs >= 0)
        .collect::<Vec<ActiveSessionsPool>>();
}

fn reduce_by_30(object : &ActiveSessionsPool) -> ActiveSessionsPool {
    return ActiveSessionsPool {
        session_id : String::from(&object.session_id),
        countdown_secs : object.countdown_secs - 30
    }
}



