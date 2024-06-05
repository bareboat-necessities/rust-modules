
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://i2cdevices.org/devices.json")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
    Ok(())
}
