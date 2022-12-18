use time::OffsetDateTime;
use wmm::declination;

fn main() {
    let date = OffsetDateTime::now_utc().date();
    let lat = 29.7363025;
    let lon = -93.8827939;
    let dec = declination(date, lat, lon).unwrap();

    println!(
        "Today's declination for coordinates {},{} is {}°",
        lat, lon, dec
    )
}