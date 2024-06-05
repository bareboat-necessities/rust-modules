use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://i2cdevices.org/devices.json")
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", resp);

    println!("type I2cDeviceInfo = (&'static str, &'static str, &'static [u8]);");
    println!();

    let mut count = 0;
    for entry in resp.as_array().unwrap() {
        if !entry["addresses"].as_array().unwrap().is_empty() {
            count += 1;
        }
    }

    println!("const I2C_SCANNER_KNOWN_DEVICES: [I2cDeviceInfo; {}] = [", count);

    for entry in resp.as_array().unwrap() {
        if !entry["addresses"].as_array().unwrap().is_empty() {
            print!("    ({}, {}, &[", entry["part_number"], entry["friendly_name"]);
            for addr in entry["addresses"].as_array().unwrap().iter().with_position() {
                if let itertools::Position::Last | itertools::Position::Only = addr.0  {
                    print!("{}", addr.1);
                } else {
                    print!("{}, ", addr.1);
                }
            }
            println!("]),");
        }
    }
    println!("];");

    Ok(())
}
