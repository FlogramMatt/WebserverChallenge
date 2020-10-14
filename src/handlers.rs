use std::convert::Infallible;

use warp::{self, http::StatusCode};

use crate::user_json::User;

/// Returns a list of users as JSON
///
/// # Arguments
///
/// * `db` - `Db` -> thread safe vector of Customer objects
pub async fn list_users() -> Result<impl warp::Reply, Infallible> {
    //get users from https://jsonplaceholder.typicode.com/users

    //let users: Vec<User> = Users.clone();
    //Ok(warp::reply::json(&users))
    let our_ids = vec![1, 3, 7, 13]; //random vector being returned for now
    Ok(warp::reply::json(&our_ids))
}

/*/// Creates a new customer
///
/// Adds a new customer object to the data store if the customer
/// doesn't already exist
///
/// # Arguments
///
/// * `new_customer` - `Customer` type
/// * `db` - `Db` -> thread safe vector of Customer objects
pub async fn create_customer(
    new_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter() {
        if customer.guid == new_customer.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    customers.push(new_customer);

    Ok(StatusCode::CREATED)
}*/

/// Gets a single user from the data store
///
/// Returns a JSON object of an existing customer. If the user
/// is not found, it returns a NOT FOUND status code.
/// # Arguments
///
/// * `guid` - String -> the id of the user to retrieve
pub async fn get_user(user_id: i32) -> Result<Box<dyn warp::Reply>, Infallible> {
    //get users from https://jsonplaceholder.typicode.com/users

/*    for user in users.iter() {
        if customer.guid == guid {
            return Ok(Box::new(warp::reply::json(&user)));
        }
    }*/

    Ok(Box::new(StatusCode::NOT_FOUND))
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
