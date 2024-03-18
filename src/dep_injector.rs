use std::sync::Arc;
use mysql::PooledConn;
use tokio::sync::Mutex;
use warp::{Filter};
use crate::data_models::{ActiveSessionsPool, AdminsData};

pub fn with_pool(pool : Arc<Mutex<PooledConn>>) -> impl Filter<Extract = (Arc<Mutex<PooledConn>>,), Error = std::convert::Infallible> + Clone { // inject the Pooled connection inside the filter
    warp::any().map(move ||  pool.clone())
}

pub fn with_admins_base(base : Arc<Mutex<Vec<AdminsData>>>) -> impl Filter<Extract = (Arc<Mutex<Vec<AdminsData>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || base.clone())
}

pub fn with_session_pool(session_pool : Arc<Mutex<Vec<ActiveSessionsPool>>>) -> impl Filter<Extract = (Arc<Mutex<Vec<ActiveSessionsPool>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || session_pool.clone())
}