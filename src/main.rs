#[macro_use]
extern crate rocket;

mod database;

pub use crate::database::db;

#[get("/")]
fn index() -> &'static str {
    let _t = db::test();
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
