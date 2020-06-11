use std::io::{stdin, stdout, Write};
use wheels::Wheels;

// fn main() {
//     let stdin = stdin();
//     //setting up stdout and going into raw mode
//     let mut stdout = stdout().into_raw_mode().unwrap();
//     //printing welcoming message, clearing the screen and going to left top corner with the cursor
//     write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
//             .unwrap();
//     stdout.flush().unwrap();

    

//     // let mut count = 0;

//     //detecting keydown events
//     for c in stdin.keys() {
//         //clearing the screen and going to top left corner
//         write!(
//             stdout,
//             "{}{}",
//             termion::cursor::Goto(1, 1),
//             termion::clear::All
//         )
//         .unwrap();

//         //i reckon this speaks for itself
//         let key = c.unwrap();
//         println!("pressed {:?}", key);
//         match key {
//             Key::Up => wheels.forward(),
//             Key::Down => wheels.backward(),
//             Key::Left => wheels.left(),
//             Key::Right => wheels.right(),
//             Key::Char(' ') | Key::Char('\n') => wheels.stop(),
//             Key::Ctrl('q') | Key::Ctrl('c') => break,
//             Key::Alt('t') => println!("termion is cool"),
//             _ => (),
//         }

//         stdout.flush().unwrap();
//     }
// }




use tonic::{transport::Server, Request, Response, Status};

use control::control_server::{Control, ControlServer};
use control::{WheelSpeeds, Empty};

pub mod control {
    tonic::include_proto!("control"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyControl {}

#[tonic::async_trait]
impl Control for MyControl {
    async fn set_speed(
        &self,
        request: Request<WheelSpeeds>, // Accept request of type HelloRequest
    ) -> Result<Response<Empty>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        Ok(Response::new(Empty {})) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    // println!("IP{}",addr.ip());
    let controller = MyControl::default();
    Server::builder()
        .add_service(ControlServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())  
}