extern crate i2cdev;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;

type I2cDeviceInfo = (&'static str, &'static str, &'static [u8]);

const I2C_SCANNER_KNOWN_DEVICES: [I2cDeviceInfo; 13] = [
    ("MPU6050", "Six-Axis (Gyro + Accelerometer) MEMS MotionTracking Devices", &[0x68, 0x69]),
    ("MPU-9250", "3-Axis Gyroscope and Accelerometer", &[0x68]),
    ("MPU-9250", "9-DoF IMU Gyroscope, Accelerometer and Magnetometer", &[0x68, 0x69]),
    ("PCF8523", "RTC", &[0x68]),
    ("BMA180", "Accelerometer", &[0x77]),
    ("BME280", "Temp/Barometric/Humidity", &[0x77, 0x76]),
    ("BME680", "Low power gas, pressure, temperature & humidity sensor", &[0x77, 0x76]),
    ("BME688", "Digital low power gas, pressure, temperature and humidity sensor with AI", &[0x77, 0x76]),
    ("BMP085", "Temp/Barometric", &[0x77]),
    ("BMP180", "Temp/Barometric", &[0x77]),
    ("BMP280", "Temp/Barometric", &[0x77, 0x76]),
    ("MS5607", "Barometric Pressure", &[0x77, 0x76]),
    ("MS5611", "Barometric Pressure", &[0x77, 0x76]),
];

fn lookup(addr: u8) {
    for i in 0 .. I2C_SCANNER_KNOWN_DEVICES.len() {
        let addresses = I2C_SCANNER_KNOWN_DEVICES[i].2;
        for j in 0 .. addresses.len() {
            if addr == addresses[j] {
                println!("  {}:  {}", I2C_SCANNER_KNOWN_DEVICES[i].0, I2C_SCANNER_KNOWN_DEVICES[i].1);
            }
        }
    }
}

fn main() {

    use crate::i2cdev::core::I2CDevice;

    for addr in 0..=127 {
        //println!("Scanning Address {:#02x}", addr as u8);

        // Scan Address
        let mut dev = LinuxI2CDevice::new("/dev/i2c-1", addr).unwrap();
        if (addr >= 0x30 && addr <= 0x37) || (addr >= 0x50 && addr <= 0x57) {
            match dev.smbus_read_byte() {
                Ok(_val) => {
                    println!("Found Address {:#02x}", addr as u8);
                    lookup(addr as u8);
                },
                Err(_err) => {
                },
            }
        }
        else {
            match dev.smbus_write_quick(false) {
                Ok(_val) => {
                    println!("Found Address {:#02x}", addr as u8);
                    lookup(addr as u8);
                },
                Err(_err) => {
                },
            }
        }
    }
}
