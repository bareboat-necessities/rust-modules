#![allow(unused_must_use)]

use notify_rust::Urgency;

fn main() {
    use notify_rust::{Notification, Hint};
    Notification::new()
        .body("Lightning\n1km")
        .icon("weather-storm")
        .appname("Weather")
        .hint(Hint::Urgency(Urgency::Critical))
        .hint(Hint::SuppressSound(false))
        .timeout(60_000_000)
        .show().unwrap();
}
