#[macro_use]
extern crate rocket;

mod database;

pub use crate::database::db;

use std::thread;

// Просмотр всех Отелелй
#[get("/getAllHotels")]
fn get_all_hotels() -> &'static str {
    let dbv = db::Db {};
    thread::spawn(|| dbv.get_hotel(3))
        .join()
        .expect("Thread panicked");
    ""
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
fn index() -> &'static str {
    thread::spawn(|| db::test())
        .join()
        .expect("Thread panicked");
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/getAllHotels", routes![get_all_hotels])
        .mount("/getAllHotels", routes![get_one_hotel])
        .mount("/getAllHotels", routes![get_all_apartments])
        .mount("/getAllHotels", routes![get_one_apartment])
        .mount("/getAllHotels", routes![insert_organization])
        .mount("/getAllHotels", routes![insert_hotel])
        .mount("/getAllHotels", routes![insert_apartment])
}
