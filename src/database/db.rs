extern crate postgres;

use chrono::NaiveDate;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;

use crate::json_struct;

use postgres::{Client, NoTls};

// Тестовое соединение без SSL, для включения SSL надо раскоментить строчку билдера и вставить билдер вместо NoTls. И настроить сервер Постгреса на работу с SSL.
pub fn test() {
    let mut builder = SslConnector::builder(SslMethod::tls_client()).unwrap();
    builder.set_ca_file("database_cert.pem").unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(
        "host=ec2-3-222-74-92.compute-1.amazonaws.com port=5432 dbname=d7742l4lf1egjv user=nibyntbbqdemhi password=c4e28ccf34294f5f66f644f983c2e5358e89b2c08a69d5d07e5e9e70e79eeee9 sslmode=require",
        connector, //NoTls,
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
            //"host=localhost port=5432 dbname=booking user=postgres",
            "host=localhost port=5432 dbname=booking user=postgres password=1qazxsw2",
            NoTls,
        )
        .unwrap()
    }

    /*id serial primary key,
    hotelId integer,
    apartmentId integer,
    dateFrom date,
    dateTo date*/
    pub fn set_booking(self, value: String) {
        println!("id = {}", value);
        let mut client = self.connecting();
        let mut query: String = String::from("insert into booking_j (booj) values ('");
        query.push_str(&value);
        query.push_str("');");
        client.execute(&query, &[&value]);
    }

    pub fn check_booking(self) {
        let mut v: Vec<json_struct::booking::Booking_j> = Vec::new();
        let mut query: String = String::from("select booj from booking_j where booj->>'id' = '1';");
        for row in self.connecting().query(&query, &[]).unwrap() {
            let booj: serde_json::Value = row.get(0);
            v.push(json_struct::booking::Booking_j { booj: booj });
        }
        let apartId = &v[0].booj["apartmentId"];
        let mut dateFrom = NaiveDate::parse_from_str(
            &str::replace(&v[0].booj["dateFrom"].to_string(), "\"", ""),
            "%Y-%m-%d",
        );
        let mut dateTo = NaiveDate::parse_from_str(
            &str::replace(&v[0].booj["dateTo"].to_string(), "\"", ""),
            "%Y-%m-%d",
        );
    }

    pub fn get_hotelj(self, id: String) -> Vec<json_struct::hotel_j::Hotel_j> {
        let mut v: Vec<json_struct::hotel_j::Hotel_j> = Vec::new();
        let mut query: String = String::from("select hotj from hotel_j");
        if id != "0" {
            query.push_str(" where hotj->>'id' = '");
            query.push_str(&id.to_string());
            query.push_str("';");
        }
        for row in self.connecting().query(&query, &[]).unwrap() {
            let hotj: serde_json::Value = row.get(0);
            v.push(json_struct::hotel_j::Hotel_j { hotj: hotj });
        }

        v
    }

    pub fn get_apartmentj(
        self,
        hotel_id: String,
        id: String,
    ) -> Vec<json_struct::apartment_j::Apartment_j> {
        let mut v: Vec<json_struct::apartment_j::Apartment_j> = Vec::new();
        let mut query: String = String::from("select apaj from apartment_j");
        let mut first_filter = 0;
        if hotel_id != "0" {
            query.push_str(" where apaj->>'hotel_id' = '");
            query.push_str(&id.to_string());
            query.push_str("';");
            first_filter += 1;
        }
        if id != "0" && first_filter == 0 {
            query.push_str(" where apaj->>'id' = '");
            query.push_str(&id.to_string());
            query.push_str("';");
        } else if id != "0" {
            query.push_str(" and apaj->>'id' = '");
            query.push_str(&id.to_string());
            query.push_str("';");
        }
        for row in self.connecting().query(&query, &[]).unwrap() {
            let hotj: serde_json::Value = row.get(0);
            v.push(json_struct::apartment_j::Apartment_j { hotj: hotj });
        }

        v
    }

    // Берем данные либо по ИД, либо все разом. Продумать пагинацию
    pub fn get_hotel(self, id: i32) -> Vec<json_struct::hotel::Hotel> {
        let mut v: Vec<json_struct::hotel::Hotel> = Vec::new();
        let mut query: String = String::from("select id, namehotel, stars from hotel");
        if id > 0 {
            query.push_str(" where id = ");
            query.push_str(&id.to_string());
        }
        for row in self.connecting().query(&query, &[]).unwrap() {
            let id: i32 = row.get(0);
            let namehotel: &str = row.get(1);
            let stars: i32 = row.get(2);

            v.push(json_struct::hotel::Hotel {
                id: id,
                name: String::from(namehotel),
                stars: stars,
            });
            println!("found hotel: {} {} {:?}", id, namehotel, stars);
        }

        v
    }

    // Берем все номера разом или по ИД. Продумать пагинацию
    pub fn get_apartment(self, id: i32) -> Vec<json_struct::apartment::Apartment> {
        let mut v: Vec<json_struct::apartment::Apartment> = Vec::new();

        let mut query: String = String::from("select id, countRoom, price from apartment");

        if id > 0 {
            query.push_str(" where id = ");
            query.push_str(&id.to_string());
        }

        for row in self.connecting().query(&query, &[]).unwrap() {
            let id: i32 = row.get(0);
            let count_room: i32 = row.get(1);
            let price: f32 = row.get(2);

            v.push(json_struct::apartment::Apartment {
                id: id,
                count_room: count_room,
                price: price,
            });
        }

        v
    }

    pub fn insert_organization(self, org: String) {
        let mut v: Vec<json_struct::organization::Organization>;
        v = serde_json::from_str(&org).unwrap();

        let mut client = self.connecting();
        for value in &v {
            println!("id = {}, name = {}", value.id, value.name);
            let mut query: String = String::from("insert into organization (orgname) values ($1)");
            client.execute(&query, &[&value.name]);
        }
    }

    pub fn insert_hotel(self, hotel: String) {
        let mut v: Vec<json_struct::hotel::Hotel>;
        v = serde_json::from_str(&hotel).unwrap();

        let mut client = self.connecting();
        for value in &v {
            println!("id = {}, name = {}", value.id, value.name);
            let mut query: String =
                String::from("insert into hotel (namehotel, stars) values ($1, $2)");
            client.execute(&query, &[&value.name, &value.stars]);
        }
    }

    pub fn insert_organizationj(self, organization: String) {
        let mut client = self.connecting();
        let mut query: String = String::from("insert into organization_j (orgj) values ('");
        query.push_str(&organization);
        query.push_str("');");

        let e = client.execute(&query, &[]);
        println!("{:?}", e);
    }

    // Работа с юзерами
    pub fn insert_user(self, user: String) -> u64 {
        let mut client = self.connecting();
        let mut query: String = String::from("insert into user_j (usej) values ('");
        query.push_str(&user);
        query.push_str("');");
        println!("{}", query);
        let e = client.execute(&query, &[]);
        println!("{:?}", e);
        let result = match e {
            Ok(res) => res,
            Err(error) => 2,
        };
        result
    }

    pub fn login(self, user: String) -> Vec<json_struct::user::User_j> {
        let mut v: Vec<json_struct::user::User_j> = Vec::new();
        let userloc: json_struct::user::UserLogin = serde_json::from_str(&user).unwrap();
        let mut query: String = String::from("select usej from user_j");
        query.push_str(" where usej->>'email' = '");
        query.push_str(&userloc.email);
        query.push_str("' and  usej->>'password' = '");
        query.push_str(&userloc.password);
        query.push_str("';");
        for row in self.connecting().query(&query, &[]).unwrap() {
            let usej: serde_json::Value = row.get(0);
            v.push(json_struct::user::User_j { usej: usej });
        }

        v
    }

    pub fn check_token(self, token: String) -> Vec<json_struct::user::User_j> {
        let mut v: Vec<json_struct::user::User_j> = Vec::new();
        let userloc: json_struct::user::UserCheck = serde_json::from_str(&token).unwrap();
        let mut query: String = String::from("select usej from user_j");
        query.push_str(" where usej->>'token' = '");
        query.push_str(&userloc.token);
        query.push_str("';");
        for row in self.connecting().query(&query, &[]).unwrap() {
            let usej: serde_json::Value = row.get(0);
            v.push(json_struct::user::User_j { usej: usej });
        }

        v
    }

    pub fn insert_hotelj(self, hotel: String) {
        let mut client = self.connecting();
        let mut query: String = String::from("insert into hotel_j (hotj) values ('");
        query.push_str(&hotel);
        query.push_str("');");

        let e = client.execute(&query, &[]);
        println!("{:?}", e);
    }

    pub fn insert_apartmentj(self, apartment: String) {
        let mut client = self.connecting();
        let mut query: String = String::from("insert into apartment_j (apaj) values ('");
        query.push_str(&apartment);
        query.push_str("');");

        let e = client.execute(&query, &[]);
        println!("{:?}", e);
    }

    pub fn insert_apartment(self, apartment: String) {
        let mut v: Vec<json_struct::apartment::Apartment>;
        v = serde_json::from_str(&apartment).unwrap();

        //postgres::types::ToSql(1, 2);

        let mut client = self.connecting();
        for value in &v {
            println!("countRoom = {}, price = {}", value.count_room, value.price);
            let mut query: String =
                String::from("insert into apartment (countRoom, price) values ($1, $2)");
            let result = client.execute(&query, &[&value.count_room, &value.price]);

            let res = match result {
                Ok(f) => {}
                Err(error) => panic!("ERR {:?}", error),
            };
        }
    }
}
