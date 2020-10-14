#[macro_use]
extern crate lazy_static;

mod routes;
mod handlers;
mod user_json;

use warp::Filter;

#[tokio::main]
async fn main(){
    //define the paths that will be exposed by the warp REST API and functions that will run when executed
    let user_routes = routes::user_routes();

    //Start the server running at localhost:3000
    warp::serve(user_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}