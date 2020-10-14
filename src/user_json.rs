use serde::{Deserialize, Serialize};

/// Represents an address
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Geo {
    /// latitude
    pub lat: String,

    /// longitude
    pub lng: String,
}

/// Represents an address
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    /// street address
    pub street: String,

    /// Suite Info aka address #2
    pub suite: String,

    /// City
    pub city: String,

    /// zipcode
    pub zipcode: String,

    ///Geo
    pub geo: Geo,
}

/// Represents an address
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Company {
    /// name of the company
    pub name: String,

    /// pitch phrase
    pub catchPhrase: String,

    /// slogan for company
    pub bs: String,
}

/// Represents a user
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    /// A unique identifier for a user record
    #[serde(default)]
    pub id: i32,

    /// Name
    pub name: String,

    /// Email address
    pub email: String,

    /// Physical address
    #[serde(default)]
    pub address: Address,

    ///user's phone number
    #[serde(default)]
    pub phone: String,

    ///User's website
    #[serde(default)]
    pub website: String,

    ///Company info
    #[serde(default)]
    pub company: Company,
}
