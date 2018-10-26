#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::clone::Clone;
use std::env;

// mod schema;
use rocket_contrib::Json;

// mod config;
mod models;
mod schema;
use self::models::User;
// use crate::models::*;

// use user::User;

#[get("/")]
fn retrieve_all_users() -> String {
    use crate::schema::users::dsl::*;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let results = users
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("user: {:?}", user);
    }

    format!("i werk retrieve all")
}

#[post("/", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    use crate::schema::users::dsl::*;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let stmt = "INSERT INTO users (id, first_name, last_name, age, hobby, email) VALUES ($1, $2, $3, $4, $5, $6)";

    // https://docs.diesel.rs/diesel/fn.insert_into.html
    // conn.execute(
    //     stmt,
    //     &[
    //         &user.id,
    //         &user.first_name,
    //         &user.last_name,
    //         &user.age,
    //         &user.hobby,
    //         &user.email,
    //     ],
    // )
    // .unwrap();

    diesel::insert_into(users)
        .values(&user.to_owned())
        //.on_conflict(user.id)
        .execute(&connection)
        .unwrap();

    user
}

#[get("/<id>")]
fn retrieve_user(id: String) -> String {
    format!("i werk retrieve {}", id)
}

#[put("/<id>")]
fn update_user(id: String) -> String {
    format!("i werk update {}", id)
}

#[delete("/<id>")]
fn delete_user(id: String) -> String {
    format!("i werk delete {}", id)
}

fn main() {
    rocket::ignite()
        .mount(
            "/users",
            routes![
                retrieve_all_users,
                create_user,
                retrieve_user,
                update_user,
                delete_user
            ],
        )
        .launch();
}
