extern crate postgres;

use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;

use postgres::{Client, NoTls};

// Тестовое соединение без SSL, для включения SSL надо раскоментить строчку билдера и вставить билдер вместо NoTls. И настроить сервер Постгреса на работу с SSL.
pub fn test() {
    /*let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file("database_cert.pem").unwrap();
    let connector = MakeTlsConnector::new(builder.build());*/

    let mut client = Client::connect(
        "host=localhost port=5432 dbname=booking user=postgres",
        NoTls,
    )
    .unwrap();

    for row in client
        .query("SELECT id, namehotel, stars FROM hotel", &[])
        .unwrap()
    {
        let id: i32 = row.get(0);
        let namehotel: &str = row.get(1);
        let stars: i32 = row.get(2);

        println!("found hotel: {} {} {:?}", id, namehotel, stars);
    }
}

pub struct Db {}

impl Db {
    pub fn connecting(self) -> Client {
        Client::connect(
            "host=localhost port=5432 dbname=booking user=postgres",
            NoTls,
        )
        .unwrap()
    }

    // Обращение к бд с лимитом. Если лимит меньше нуля, то выдаст все данные, иначе, выдаст с учетом лимита.
    pub fn get_hotel(self, limit: i32) {
        let mut query: String;
        if limit < 0 {
            query = String::from("select id, namehotel, stars from hotel");
        } else {
            query = String::from("select id, namehotel, stars from hotel limit ");
            query.push_str(&limit.to_string())
        }
        for row in self.connecting().query(&query, &[]).unwrap() {
            let id: i32 = row.get(0);
            let namehotel: &str = row.get(1);
            let stars: i32 = row.get(2);

            println!("found hotel: {} {} {:?}", id, namehotel, stars);
        }
    }

    pub fn get_apartment(all_hotel: bool) {}

    pub fn insert_organization(all_hotel: bool) {}

    pub fn insert_hotel(all_hotel: bool) {}

    pub fn insert_apartment(all_hotel: bool) {}
}
