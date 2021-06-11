#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod core;
pub mod service;

extern crate chrono;

use chrono::Utc;

pub fn now_utc() -> chrono::DateTime<Utc> {
    Utc::now()
} 

