use std::convert::Infallible;
use std::io::Read;

use warp::{self, http::StatusCode};

use lazy_static;

use reqwest;
use regex::Regex;
use crate::user_json::User;

lazy_static! {
    static ref email_regex: Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    static ref user_vector_as_db: Vec<User> = Vec::with_capacity(20);
}

/// Returns a list of users as JSON
///
pub async fn list_users() -> Result<impl warp::Reply, warp::Rejection> {
    //get users from https://jsonplaceholder.typicode.com/users

    //for a real project I would use nonblocking and handle this asynchronously
    //also I wouldn't use unwrap which might panic and crash the entire server
    let res = reqwest::get("https://jsonplaceholder.typicode.com/users").await.unwrap();

    let body = res.text().await.unwrap();

    Ok(warp::reply::with_status(body,StatusCode::OK))
}

/// Gets a single user from the data store
///
/// Returns a JSON object of an existing customer. If the user
/// is not found, it returns a NOT FOUND status code.
/// # Arguments
///
/// * `user_id` - i32 -> the id of the user to retrieve
pub async fn get_user(user_id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    //get users from https://jsonplaceholder.typicode.com/users

    let mut url:String = "https://jsonplaceholder.typicode.com/users/".to_string();
    url.push_str(user_id.to_string().as_str());

    let res = reqwest::get(url.as_str()).await.unwrap();
    let body = res.text().await.unwrap();

    if(body == "{}"){
        let return_string = "Not Found".to_string();
        return Ok(warp::reply::with_status(return_string, StatusCode::NOT_FOUND));
    }

    Ok(warp::reply::with_status(body, StatusCode::OK))
    /*let users: Vec<User> = serde_json::from_str(body.as_str()).unwrap();

    for user in users.iter()
    {
        if(user.id == user_id){
            let user_string = serde_json::to_string(user).unwrap();

            Ok(warp::reply::with_status(user_string, StatusCode::OK))
        }
    }*/

    //Ok(Box::new(StatusCode::NOT_FOUND))
}

/// Creates a new user
///
/// Create a new user and add to our virtual database
///
/// # Arguments
///
/// * `new_customer` - `Customer` type
pub async fn create_user(
    mut new_user: User,
) -> Result<impl warp::Reply, Infallible> {

    let mut failed_flag: bool = false;
    let mut errors: Vec<String>=Vec::with_capacity(2);

    println!("{}",serde_json::to_string(&new_user).unwrap().as_str());

    //validate username is long enough
    if new_user.name.len() < 3 {
        errors.push("Name too short".to_string());
        failed_flag = true;
        //Ok(warp::reply::with_status("Name too short".to_string(), StatusCode::));
    }

    //check if email_regex is long enough
    if(!email_regex.is_match(new_user.email.as_str())){
        errors.push("Invalid email address".to_string());
        failed_flag = true;
    }

    if(failed_flag){
        let errors_string:String = serde_json::to_string(&errors).unwrap();
        return Ok(warp::reply::with_status(errors_string, StatusCode::UNPROCESSABLE_ENTITY));
    }

    new_user.id=11;

    let user_string:String = serde_json::to_string(&new_user).unwrap();
    Ok(warp::reply::with_status(user_string, StatusCode::OK))
}

/*/// Updates an existing customer
///
/// Overwrites an existing customer in the data store and returns
/// an OK status code. If the customer is not found, a NOT FOUND status
/// code is returned.
///
/// # Arguments
///
/// * `updated_customer` - `Customer` -> updated customer info
/// * `db` - `Db` -> thread safe data store
pub async fn update_customer(
    guid: String,
    updated_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter_mut() {
        if customer.guid == guid {
            *customer = updated_customer;
            return Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::NOT_FOUND)
}

/// Deletes a customer from the data store
///
/// If the customer exists in the data store, the customer is
/// removed and a NO CONTENT status code is returned. If the customer
/// does not exist, a NOT FOUND status code is returned.
///
/// # Arguments
///
/// * `guid` - String -> the id of the customer to delete
/// * `db` - `Db` -> thread safe data store
pub async fn delete_customer(guid: String) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    let customer_count = customers.len();

    customers.retain(|customer| customer.guid != guid);

    let deleted = customers.len() != customer_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}*/
