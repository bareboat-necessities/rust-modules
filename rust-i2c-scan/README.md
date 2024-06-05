# i2c-scanner for Linux written in Rust

Detects i2c addresses using roughly same way as i2cdetect.

Checks i2c database to display possible matching devices models by their found i2c addresses.

(The code was generated based on data in https://i2cdevices.org database)

## Building

````
cd rust-i2c-scan
cargo build
````

## Sample output

````
bash# rust-i2c-scan
````

````
Found Address 0x68
  AMG8833:  IR Thermal Camera Breakout
  BQ32000:  Real-Time Clock (RTC)
  DS1307:  64 x 8 Serial Real-Time Clock
  DS1371:  I2C, 32-Bit Binary Counter Watchdog Clock
  DS3231:  Extremely Accurate RTC/TCXO/Crystal
  ICM-20948:   9-Axis Motion Tracking device
  ITG3200:  Gyro
  LTC4151:  High voltage (7-80V) current and voltage monitor 
  MCP3422:  18-Bit, Multi-Channel ΔΣ Analog-to-Digital Converter with I2CTM Interface and On-Board Reference
  MPU6050:  Six-Axis (Gyro + Accelerometer) MEMS MotionTracking™ Devices
  MPU-9250:  9-DoF IMU Gyroscope, Accelerometer and Magnetometer
  MPU-9250:  3-Axis Gyroscope and Accelerometer
  PCA9685:  16-channel PWM driver default address
  PCF8523:  RTC
  PCF8573:  Clock/calendar with Power Fail Detector
  WITTY PI 3:  WITTY PI 3 (Mini) - REALTIME CLOCK (DS3231SN) AND POWER MANAGEMENT FOR RASPBERRY PI
Found Address 0x76
  BME280:  Temp/Barometric/Humidity
  BME680:  Low power gas, pressure, temperature & humidity sensor
  BME688:  Digital low power gas, pressure, temperature and humidity sensor with AI
  BMP280:  Temp/Barometric
  HT16K33:  LED Matrix Driver
  MS5607:  Barometric Pressure
  MS5611:  Barometric Pressure
  PCA9539:  16-bit I/O expander with interrupt and reset
  PCA9541:  2-1 I2C bus arbiter
  PCA9685:  16-channel PWM driver default address
  SPL06-007:  Digital Temperature/Pressure Sensor
  TCA9548:  1-to-8 I2C Multiplexer
  TCA9548A:  Low-Voltage8-Channel I2CSwitchwithReset
  XD8574:  I²C 8-Bit I/O Expander
Found Address 0x77
  BMA180:  Accelerometer
  BME280:  Temp/Barometric/Humidity
  BME680:  Low power gas, pressure, temperature & humidity sensor
  BME688:  Digital low power gas, pressure, temperature and humidity sensor with AI
  BMP085:  Temp/Barometric
  BMP180:  Temp/Barometric
  BMP280:  Temp/Barometric
  HT16K33:  LED Matrix Driver
  IS31FL3731:  144-LED Audio Modulated Matrix LED Driver (CharliePlex)
  MS5607:  Barometric Pressure
  MS5611:  Barometric Pressure
  PCA9539:  16-bit I/O expander with interrupt and reset
  PCA9541:  2-1 I2C bus arbiter
  PCA9685:  16-channel PWM driver default address
  SPL06-007:  Digital Temperature/Pressure Sensor
  TCA9548:  1-to-8 I2C Multiplexer
  TCA9548A:  Low-Voltage8-Channel I2CSwitchwithReset
  XD8574:  I²C 8-Bit I/O Expander
````