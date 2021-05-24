#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate js_sys;
extern crate chrono;

use js_sys::Date;
use chrono::offset::Local;

pub fn now_utc() -> f64 {
    if cfg!(target_arch="wasm32") {
        Date::now()
    } else {
        Local::now().timestamp_millis() as f64
    }
} 

