#[macro_use]
extern crate log;

use futures::prelude::*;
use gpsd_proto::UnifiedResponse;
use futures::StreamExt;
use tokio_util::codec::Framed;
use tokio_util::codec::LinesCodec;
use tokio::net::TcpStream;
use std::error::Error;
use std::net::{SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let saddr: SocketAddr = "127.0.0.1:2947".parse().unwrap();
    let conn = TcpStream::connect(saddr).await?;

    let mut framed = Framed::new(conn, LinesCodec::new_with_max_length(64 * 1024));
    framed.send(gpsd_proto::ENABLE_WATCH_CMD.to_string()).await?;
    while let Some(Ok(line)) = framed.next().await {
        match serde_json::from_str(line.as_str()) {
            Ok(rd) => match rd {
                UnifiedResponse::Version(v) => {
                    if v.proto_major < gpsd_proto::PROTO_MAJOR_MIN {
                        panic!("Gpsd major version mismatch");
                    }
                    info!("Gpsd version {} connected", v.rev);
                }
                UnifiedResponse::Devices(_) => {}
                UnifiedResponse::Watch(_) => {}
                UnifiedResponse::Device(d) => info!("Device {:?}", d),
                UnifiedResponse::Tpv(t) => info!("Tpv {:?}", t),
                UnifiedResponse::Sky(s) => info!("Sky {:?}", s),
                UnifiedResponse::Pps(p) => info!("PPS {:?}", p),
                UnifiedResponse::Gst(g) => info!("GST {:?}", g),
            },
            Err(e) => {
                error!("Error decoding: {}", e);
            }
        };
    }

    Ok({})
}
