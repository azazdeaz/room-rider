use control::control_client::ControlClient;
use control::WheelSpeeds;
use pasts::{CvarExec, prelude::*};
use stick::{Event, Gamepad, Port};
use tonic::transport::Channel;
use std::error::Error;


use gilrs::{Gilrs, Button, EventType, Axis};



async fn event_loop(client: &mut ControlClient<Channel>) {
    let mut port = Port::new();
    let mut gamepads = Vec::<Gamepad>::new();
    loop {
        match [port.fut(), gamepads.select().fut()]
            .select()
            .await
            .1
        {
            (_, Event::Connect(gamepad)) => {
                println!(
                    "Connected p{}, id: {:X}, name: {}",
                    gamepads.len() + 1,
                    gamepad.id(),
                    gamepad.name(),
                );
                gamepads.push(*gamepad);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id + 1);
                gamepads.swap_remove(id);
            }
            (id, Event::MotionV(speed)) => {
                println!("speed {:?} {:?}", id, speed);
                
                println!("client {:?}", client);
                let request = tonic::Request::new(WheelSpeeds {
                    left: speed,
                    right: speed,
                });
                
                println!("ready {:?}", client);
                let response = client.set_speed(request).await.unwrap();

                println!("RESPONSE={:?}", response);
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::Accept(pressed) => {
                        gamepads[id].rumble(if pressed {
                            0.25
                        } else {
                            0.0
                        });
                    }
                    Event::Cancel(pressed) => {
                        gamepads[id].rumble(if pressed {
                            1.0
                        } else {
                            0.0
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

pub mod control {
    tonic::include_proto!("control");
}
pub mod things {
    tonic::include_proto!("things");
}
fn axes_to_speeds(joi_x: f32, joi_y: f32) -> (f32, f32) {
    let pivot_y_limit: f32 = 0.25;
			
    // TEMP VARIABLES
    let mut mot_premix_l: f32;    // Motor (left)  premixed output        (-128..+127)
    let mut mot_premix_r: f32;    // Motor (right) premixed output        (-128..+127)
    let mut pivot_speed: f32;      // Pivot Speed                          (-128..+127)
    let mut pivot_scale: f32;      // Balance scale b/w drive and pivot    (   0..1   )


    // Calculate Drive Turn output due to Joystick X input
    if joi_y >= 0.0 {
        // Forward
        mot_premix_l = if joi_x >= 0.0 {1.0 }else{ 1.0 + joi_x};
        mot_premix_r = if joi_x >= 0.0 {1.0 - joi_x }else{ 1.0};
    } else {
        // Reverse
        mot_premix_l = if joi_x >= 0.0 {1.0 - joi_x }else{ 1.0};
        mot_premix_r = if joi_x >= 0.0 {1.0 }else{ 1.0 + joi_x};
    }

    // Scale Drive output due to Joystick Y input (throttle)
    mot_premix_l = mot_premix_l * joi_y;
    mot_premix_r = mot_premix_r * joi_y;

    // Now calculate pivot amount
    // - Strength of pivot (pivot_speed) based on Joystick X input
    // - Blending of pivot vs drive (pivot_scale) based on Joystick Y input
    pivot_speed = joi_x;
    pivot_scale = if joi_y.abs() > pivot_y_limit  {0.0} else {1.0 - joi_y.abs()/pivot_y_limit};

    // Calculate final mix of Drive and Pivot
    let left = (1.0-pivot_scale)*mot_premix_l + pivot_scale*( pivot_speed);
    let right = (1.0-pivot_scale)*mot_premix_r + pivot_scale*(-pivot_speed);
    (left, right)
}

async fn send_command(joi_x: f32, joi_y: f32, client: &mut ControlClient<Channel>) {
    let (left, right) = axes_to_speeds(joi_x, joi_y);
    println!("speed {} {}", left, right);
    let request = tonic::Request::new(WheelSpeeds {
        left,
        right,
    });
    client.set_speed(request).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ControlClient::connect("http://192.168.50.222:50051").await?;
    // let mut client = ControlClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(WheelSpeeds {
        left: 0.0,
        right: 0.0,
    });
    println!("stop...");
    client.set_speed(request).await.unwrap();

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut joi_x = 0.0;
    let mut joi_y = 0.0;

    let small_to_zero = |x: f32| if x.abs() < 0.01 { 0.0 } else { x };

    loop {
        // Examine new events
        let mut changed = false;
        while let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);
            if let EventType::AxisChanged(Axis::LeftStickX, speed, _) = event {
                changed = true;
                joi_x = small_to_zero(speed);
                // println!("X={:?}", speed);
                // send_command(joi_x, joi_y, &mut client).await;
            }
            if let EventType::AxisChanged(Axis::LeftStickY, speed, _) = event {
                changed = true;
                joi_y = small_to_zero(speed);
                // println!("Y={:?}", speed);
                // send_command(joi_x, joi_y, &mut client).await;
            }
        }

        if changed {
            send_command(joi_x, joi_y, &mut client).await;
        }
    }

    Ok(())
}

// use control::control_client::ControlClient;
// use control::WheelSpeeds;

// pub mod control {
//     tonic::include_proto!("control");
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = ControlClient::connect("http://192.168.50.222:50051").await?;

//     let request = tonic::Request::new(WheelSpeeds {
//         left: 0.8,
//         right: 0.0,
//     });

//     let response = client.set_speed(request).await?;

//     println!("RESPONSE={:?}", response);

//     Ok(())
// }