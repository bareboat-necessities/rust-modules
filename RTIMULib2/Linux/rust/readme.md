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
````


* cargo test

````
cd ~/RTIMULib2/Linux/rust
cargo test
````


## Rust Code

The Rust equivalent for the following C++ code


````
MyClass instance = MyClass();
instance.method();
````

is this:
````
let instance = std::mem::MaybeUninit::<MyClass>::uninit();
MyClass_MyClass(instance.as_mut_ptr());
instance.assume_init_mut().method();
````



