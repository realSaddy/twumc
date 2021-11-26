pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use hmac::{Hmac, NewMac};
use jwt::{Error, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn sign_jwt(user: models::User) -> Result<String, Error> {
    let jwt_key: Hmac<Sha256> = Hmac::new_from_slice(
        env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .expect("Failed creating HMAC for JWT!");
    let mut data = BTreeMap::new();
    data.insert("id", user.id.to_string());
    data.insert("name", user.name);
    data.insert("permission", user.permission.to_string());
    return data.sign_with_key(&jwt_key);
}
