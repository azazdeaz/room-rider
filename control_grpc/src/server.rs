use std::io::{stdin, stdout, Write};
use wheels::Wheels;
use std::sync::{Arc, Mutex};




use tonic::{transport::Server, Request, Response, Status};

use control::control_server::{Control, ControlServer};
use control::{WheelSpeeds, Empty};

pub mod control {
    tonic::include_proto!("control"); // The string specified here must match the proto package name
}

// #[derive(Debug, Default)]
pub struct MyControl 
    wheels: Arc<Mutex<Wheels>>,
}

#[tonic::async_trait]
impl Control for MyControl {
    async fn set_speed(
        &self,
        request: Request<WheelSpeeds>, // Accept request of type HelloRequest
    ) -> Result<Response<Empty>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        // let mut wheels = Wheels::new();
        let msg = request.get_ref();
        // wheels.speed(msg.left as f64, msg.right as f64);
        self.wheels.clone().lock().unwrap().speed(msg.left as f64, msg.right as f64);
        println!("cmd sent");
        Ok(Response::new(Empty {})) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wheels = Wheels::new();
    wheels.stop();
    let addr = "0.0.0.0:50051".parse()?;
    // println!("IP{}",addr.ip());
    let controller = MyControl{ wheels: Arc::new(Mutex::new(wheels)) };
    Server::builder()
        .add_service(ControlServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())  
}