use as3935_bbn::interface::i2c::I2cAddress;
use as3935_bbn::{
    Event, HeadOfStormDistance, InterfaceSelection, ListeningParameters, SensorPlacing,
    SignalVerificationThreshold, AS3935,
};
use chrono::Utc;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use simple_signal::{set_handler, Signal};
use std::sync::mpsc::channel;

fn alert(msg: String) {
    #![allow(unused_must_use)]
    use notify_rust::{Notification, Hint, Urgency};
    Notification::new()
        .body(msg.as_str())
        .icon("weather-storm")
        .appname("Weather")
        .hint(Hint::Urgency(Urgency::Critical))
        .hint(Hint::SuppressSound(false))
        .timeout(60_000_000)
        .show().unwrap();
}

fn main() {
    simple_logger::init().unwrap();

    println!("Initializing…");

    let gpio = Gpio::new().unwrap();

    let mut as3935 = AS3935::new(
        InterfaceSelection::I2c(I2c::with_bus(1).unwrap(), I2cAddress::new(3)),
        gpio.get(24).unwrap().into_input(),
    ).unwrap();

    println!("Starting to listen…");

    let events = as3935
        .listen(
            ListeningParameters::default()
                .with_sensor_placing(SensorPlacing::Indoor)
                .with_signal_verification_threshold(SignalVerificationThreshold::new(5).unwrap()),
        ).unwrap();

    println!("Listening for events…");

    std::thread::spawn(move || {
        for event in events {
            alert(format!(
                "[{}] {}",
                Utc::now().to_rfc3339(),
                match event {
                    Event::Lightning(lightning) => format!(
                        "Lightning detected: {}.",
                        match lightning {
                            HeadOfStormDistance::Kilometers(km) => format!("{} km", km),
                            HeadOfStormDistance::OutOfRange => String::from("out of range"),
                            HeadOfStormDistance::Overhead => String::from("overhead"),
                        }
                    ),
                    Event::Noise => String::from("Noise detected."),
                    Event::Disturbance => String::from("Disturber detected."),
                }
            ))
        }
    });

    let (tx, rx) = channel::<()>();

    set_handler(&[Signal::Term, Signal::Int], move |_signals| {
        tx.send(()).unwrap();
    });

    rx.recv().unwrap();

    println!("Terminating…");
    as3935.terminate().unwrap();

    println!("Terminated.");
}

