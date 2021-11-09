#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod utils;

use diesel::prelude::*;
use models::*;
use schema::users::dsl::*;
use utils::establish_connection;

fn main() {
    // コネクションを取得する
    let connection = establish_connection();

    // ユーザ一覧を取得する
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    // ユーザ一覧の表示
    println!("Displaying {} users", results.len());
    for r in results {
        println!("{:?}", r);
    }
}
