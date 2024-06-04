extern crate i2cdev;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;

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
                },
                Err(_err) => {
                },
            }
        }
        else {
            match dev.smbus_write_quick(false) {
                Ok(_val) => {
                    println!("Found Address {:#02x}", addr as u8);
                },
                Err(_err) => {
                },
            }
        }
    }
}
