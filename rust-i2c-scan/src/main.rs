extern crate i2cdev;

// NOTE: It doesn't work

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;

fn main() {

    use crate::i2cdev::core::I2CTransfer;

    for addr in 1..=127 {
        println!("Scanning Address {:#02x}", addr as u8);

        // Scan Address
        let mut dev = LinuxI2CDevice::new("/dev/i2c-1", addr);
        let mut read_data = [0; 2];
        let mut msgs = [
            LinuxI2CMessage::write(&[0x01]),
            LinuxI2CMessage::read(&mut read_data).with_address(addr as u16)
        ];
        dev.expect("Reason").transfer(&mut msgs);
        println!("Read: {:?}", read_data);
    }
}
