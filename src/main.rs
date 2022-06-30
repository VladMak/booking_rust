#[macro_use]
extern crate rocket;

mod database;
mod json_struct;
mod models;

pub use crate::database::db;

use std::thread;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Task {
    name: i32,
    surname: i32,
}

// В случае если ID = 0 или меньше нуля, то получим все отели
#[get("/getHotel/<id>")]
fn get_hotel(id: i32) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_hotel(id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[get("/getApartment/<id>")]
fn get_apartment(id: i32) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_apartment(id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[get("/getHotelj/<id>")]
fn get_hotelj(id: i32) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_hotelj(id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[put("/setBooking", format = "application/json", data = "<data>")]
fn set_booking(data: String) -> String {
    let worker = models::worker::Worker {};
    worker.check_booking(&data);
    let dbv = db::Db {};
    println!("{}", data);
    thread::spawn(move || dbv.set_booking(data))
        .join()
        .expect("Thread panicked");
    String::from("true")
}

// АДМИН ПАНЕЛЬ
// Добавление Юр. лица
#[put("/insertOrganization", format = "application/json", data = "<user>")]
fn insert_organization(user: String) -> String {
    let dbv = db::Db {};
    println!("{}", user);
    thread::spawn(move || dbv.insert_organization(user))
        .join()
        .expect("Thread panicked");
    String::from("true")
}

// Добавление Отеля
#[put("/insertHotelj", format = "application/json", data = "<hotel>")]
fn insert_hotelj(hotel: String) -> String {
    let dbv = db::Db {};
    println!("{}", hotel);
    thread::spawn(move || dbv.insert_hotelj(hotel))
        .join()
        .expect("Thread panicked");
    String::from("true")
}

// Добавление Отеля
#[put("/insertHotel", format = "application/json", data = "<hotel>")]
fn insert_hotel(hotel: String) -> String {
    let dbv = db::Db {};
    println!("{}", hotel);
    thread::spawn(move || dbv.insert_hotel(hotel))
        .join()
        .expect("Thread panicked");
    String::from("true")
}

// Добавление номеров
// 20 фото для номера
#[put("/insertApartment", format = "application/json", data = "<apartment>")]
fn insert_apartment(apartment: String) -> String {
    let dbv = db::Db {};
    println!("{}", apartment);
    thread::spawn(move || dbv.insert_apartment(apartment))
        .join()
        .expect("Thread panicked");
    String::from("true ap")
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
        .mount("/", routes![get_apartment])
        .mount("/", routes![insert_organization])
        .mount("/", routes![insert_hotel])
        .mount("/", routes![insert_apartment])
        .mount("/", routes![get_hotel])
        .mount("/", routes![insert_hotelj])
        .mount("/", routes![get_hotelj])
        .mount("/", routes![set_booking])
}
