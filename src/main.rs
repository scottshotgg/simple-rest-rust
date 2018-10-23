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
    format!("i werk retrieve all")
}

#[post("/", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
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
    use crate::schema::users::dsl::*;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let results = users
        .filter(id.eq(5))
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------\n");
        println!("{}", post.first_name);
    }

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
