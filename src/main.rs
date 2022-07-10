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
fn get_hotel(id: String) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_hotelj(id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[get("/getApartment/<hotel_id>/<id>")]
fn get_apartment(hotel_id: String, id: String) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_apartmentj(hotel_id, id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[get("/getHotelj/<id>")]
fn get_hotelj(id: String) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_hotelj(id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[get("/getBooking/<id>")]
fn get_booking(id: String) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.get_booking(id))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[put("/setBooking", format = "application/json", data = "<data>")]
fn set_booking(data: String) -> String {
    //let worker = models::worker::Worker {};
    //worker.check_booking(&data);
    let dbv = db::Db {};
    /*thread::spawn(move || dbv.check_booking())
    .join()
    .expect("Thread panicked");*/
    println!("{}", data);
    let result = thread::spawn(move || dbv.set_booking(data))
        .join()
        .expect("Thread panicked");
    if result == 1 {
        String::from("ok")
    } else {
        String::from("err")
    }
}

// АДМИН ПАНЕЛЬ
// Добавление Юр. лица
#[put("/insertOrganization", format = "application/json", data = "<user>")]
fn insert_organization(user: String) -> String {
    let dbv = db::Db {};
    println!("{}", user);
    thread::spawn(move || dbv.insert_organizationj(user))
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
    thread::spawn(move || dbv.insert_hotelj(hotel))
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
    thread::spawn(move || dbv.insert_apartmentj(apartment))
        .join()
        .expect("Thread panicked");
    String::from("true ap")
}

#[put("/insertUser", format = "application/json", data = "<user>")]
fn insert_user(user: String) -> String {
    let dbv = db::Db {};
    println!("{}", user);
    let result = thread::spawn(move || dbv.insert_user(user))
        .join()
        .expect("Thread panicked");
    if result == 1 {
        String::from("ok")
    } else {
        String::from("err")
    }
}

#[get("/login/<email>/<password>", format = "application/json")]
fn login(email: String, password: String) -> String {
    let dbv = db::Db {};
    println!("{} {}", email, password);
    let res = thread::spawn(move || dbv.login(email, password))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
}

#[get("/checkToken/<token>", format = "application/json")]
fn get_user(token: String) -> String {
    let dbv = db::Db {};
    let res = thread::spawn(move || dbv.check_token(token))
        .join()
        .expect("Thread panicked");
    let j = serde_json::to_string(&res).unwrap();
    j
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
        .mount("/", routes![get_booking])
        .mount("/", routes![set_booking])
        .mount("/", routes![insert_user])
        .mount("/", routes![get_user])
        .mount("/", routes![login])
}
