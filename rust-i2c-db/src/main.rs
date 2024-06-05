
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://i2cdevices.org/devices.json")
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
