use crate::json_struct::booking;

pub struct Worker {}

impl Worker {
    pub fn check_booking(self, value: &String) {
        let t: booking::Booking = serde_json::from_str(value).unwrap();
        println!("{}, {}", t.id, t.hotelId);
    }
}
