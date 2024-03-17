use warp::cors::Builder;
use warp::http::Method;

pub fn get() -> Builder {
    return warp::cors() // Access-Control-Allow-Origin for a requests from the web.
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Origin", "Origin", "Accept", "X-Requested-With", "Content-Type"])
        .allow_methods(&[Method::GET, Method::POST]);
}

pub fn get_cors_key<'a>() -> &'a str {
    return "Access-Control-Allow-Origin"
}

pub fn get_cors_value<'a>() -> &'a str {
    return "*"
}