use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;
use crate::user_json::User;

/// All user routes
pub fn user_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_user()
        .or(create_user())
        .or(users_list())
}

/// GET /users
/// Get all users
fn users_list(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api"/"v1"/"users")
        .and(warp::get())
        .and_then(handlers::list_users)
}

/// GET /users/{guid}
/// Gets a specific user from server
fn get_user(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api"/"v1"/"users"/i32)
        .and(warp::get())
        .and_then(handlers::get_user)
}

/// POST /users
/// must use header: "Content-Type: application/json"
/// JSON body must include "name" and "email"
/// name must be at least 3 digits in length
/// email must be of valid format as verified by Regex
fn create_user(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api"/"v1"/"users")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::create_user)
}

///Needed by POST to convert incoming JSON into a User object
fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(2048 * 16).and(warp::body::json())
}

/*/// POST /users
fn create_user(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_customer)
}

/// PUT /users/{guid}
fn update_user(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_customer)
}

/// DELETE /users/{guid}
fn delete_user(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_customer)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}*/


