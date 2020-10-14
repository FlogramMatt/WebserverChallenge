use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;
use crate::user_json::User;

/// All user routes
pub fn user_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_user()
        //.or(update_customer(db.clone()))
        //.or(delete_customer(db.clone()))
        //.or(create_customer(db.clone()))
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

/*fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}*/

/*/// POST /customers
fn create_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_customer)
}

/// PUT /customers/{guid}
fn update_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_customer)
}

/// DELETE /customers/{guid}
fn delete_customer(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_customer)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}*/


