# Rust bindings for RTIMULib2

Rust bindings for RTIMULib2 generated with https://rust-lang.github.io/rust-bindgen/


## Steps to generate bindings:


* On Debian

````
sudo apt install llvm-dev libclang-dev clang
````

* clone and build RTIMULib2

````
cd ~
git clone https://github.com/seandepagnier/RTIMULib2
cd RTIMULib2/RTIMULib
mkdir build && cd build 
cmake ..
make
````

* put content of this directory under RTIMULib2/Linux/rust/

````
cd ~
git clone https://github.com/bareboat-necessities/rust-modules
cp -r rust-modules/RTIMULib2/Linux/rust ~/RTIMULib2/Linux/
````

* cargo build


````
cd ~/RTIMULib2/Linux/rust
cargo build
find . -name bindings.rs
````


* cargo test

````
cd ~/RTIMULib2/Linux/rust
cargo test
````


## Rust Code

The Rust code example:


````
use std::{ptr, thread, time};
use std::ffi::CStr;
use std::mem::MaybeUninit;
use time::Duration;

use rtimulib_rust::{RTFusionKalman4_RTFusionKalman4_destructor, RTIMU, RTIMUMPU925x_IMUGetPollInterval, RTIMUMPU925x_IMUInit, RTIMUMPU925x_IMURead, RTIMUMPU925x_RTIMUMPU925x_destructor, RTIMUSettings, RTIMUSettings_RTIMUSettings, RTMath_displayDegrees};

fn main() {
    println!("Hello RTIMULib!");

    let mut instance = MaybeUninit::<RTIMUSettings>::uninit();
    let settings = instance.as_mut_ptr();
    unsafe {
        RTIMUSettings_RTIMUSettings(settings,"\0".as_ptr());
        (*settings).m_fusionType = 1;
        let imu = RTIMU::createIMU(settings);
        RTIMUMPU925x_IMUInit(imu as *mut _);

        let fusion = (*imu).m_fusion;
        (*fusion).m_slerpPower = 0.02;
        (*fusion).m_enableGyro = true;
        (*fusion).m_enableAccel = true;
        (*fusion).m_enableCompass = true;

        let poll = RTIMUMPU925x_IMUGetPollInterval(imu as *mut _);
        println!("Poll: {}", poll);

        thread::sleep(Duration::from_millis((poll * 1000) as _));

        RTIMUMPU925x_IMURead(imu as *mut _);
        let mut imu_data = (*imu).m_imuData;
        let p_pose = ptr::addr_of_mut!(imu_data.fusionPose);

        let deg = RTMath_displayDegrees("Deg\0".as_ptr(), p_pose);
        println!("{}", CStr::from_ptr(deg).to_str().unwrap().to_owned());

        RTFusionKalman4_RTFusionKalman4_destructor(fusion as *mut _);
        RTIMUMPU925x_RTIMUMPU925x_destructor(imu as *mut _);
    }
}


````


