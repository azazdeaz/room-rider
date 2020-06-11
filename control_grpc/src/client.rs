use control::control_client::ControlClient;
use control::WheelSpeeds;

pub mod control {
    tonic::include_proto!("control");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ControlClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(WheelSpeeds {
        left: 0.8,
        right: 0.0,
    });

    let response = client.set_speed(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}