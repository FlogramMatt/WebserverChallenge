use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("apt" / "v1" / "users" / String).and_then(get_user);
        //.method("GET");
        //.map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

pub async fn get_user(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, {}!", name);
     println!("{}", &reply);
    Ok(warp::reply::html(reply))
}