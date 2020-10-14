#[macro_use]
extern crate lazy_static;

mod routes;
mod handlers;
mod user_json;

use warp::Filter;

#[tokio::main]
async fn main(){
    let user_routes = routes::user_routes();

    warp::serve(user_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}