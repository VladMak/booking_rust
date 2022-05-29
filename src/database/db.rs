extern crate postgres;

use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;

use postgres::{Client, NoTls};

pub fn test() -> Result<(), postgres::Error> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file("database_cert.pem")?;
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost/booking",
        connector,
    )?;

    for row in client.query("SELECT id, namehotel, stars FROM hotel", &[])? {
        let id: i32 = row.get(0);
        let namehotel: &str = row.get(1);
        let stars: i32 = row.get(2);

        println!("found hotel: {} {} {:?}", id, namehotel, stars);
    }

    Ok(())
}
