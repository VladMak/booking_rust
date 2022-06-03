#[macro_use]
extern crate rocket;

mod database;
mod json_struct;

pub use crate::database::db;

use std::thread;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Task {
    name: i32,
    surname: i32,
}

// Просмотр всех Отелелй
#[get("/getAllHotels")]
fn get_all_hotels() -> String {
    let dbv = db::Db {};
    let res = thread::spawn(|| dbv.get_hotel(3))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

// Просмотр одного Отеля
#[get("/getOneHotel")]
fn get_one_hotel() -> &'static str {
    ""
}

// Просмотр всех номеров
#[get("/getAllApartments")]
fn get_all_apartments() -> &'static str {
    ""
}

// Просмотр одного номера
#[get("/getOneApartment")]
fn get_one_apartment() -> &'static str {
    ""
}

// АДМИН ПАНЕЛЬ
// Добавление Юр. лица
#[get("/insertOrganization")]
fn insert_organization() -> &'static str {
    ""
}

// Добавление Отеля
#[get("/insertHotel")]
fn insert_hotel() -> &'static str {
    ""
}

// Добавление номеров
// 20 фото для номера
#[get("/insertApartment")]
fn insert_apartment() -> &'static str {
    ""
}

#[get("/")]
fn index() -> String {
    let test = Task {
        name: 32,
        surname: 50,
    };
    let j = serde_json::to_string(&test).unwrap();
    j
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_all_hotels])
        .mount("/", routes![get_one_hotel])
        .mount("/", routes![get_all_apartments])
        .mount("/", routes![get_one_apartment])
        .mount("/", routes![insert_organization])
        .mount("/", routes![insert_hotel])
        .mount("/", routes![insert_apartment])
}
