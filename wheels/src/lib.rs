use rust_gpiozero::{Motor, LED};

const GPIO_EEP: u8 = 26;
const GPIO_IN1: u8 = 5;
const GPIO_IN2: u8 = 6;
const GPIO_IN3: u8 = 13;
const GPIO_IN4: u8 = 19;

pub struct Wheels {
    left: Motor,
    right: Motor,
    eep: LED,
}

impl Wheels {
    pub fn new() -> Wheels {
        Wheels {
            eep: LED::new(GPIO_EEP),
            left: Motor::new(GPIO_IN4, GPIO_IN3),
            right: Motor::new(GPIO_IN2, GPIO_IN1),
        }
    }
    pub fn forward(&mut self) {
        self.eep.on();
        self.left.forward();
        self.right.forward();
        println!("forward");
    }
    pub fn backward(&mut self) {
        self.eep.on();
        self.left.backward();
        self.right.backward();
        println!("backward");
    }
    pub fn left(&mut self) {
        self.eep.on();
        self.left.backward();
        self.right.forward();
        println!("left");
    }
    pub fn right(&mut self) {
        self.eep.on();
        self.left.forward();
        self.right.backward();
        println!("right");
    }
    pub fn stop(&mut self) {
        self.eep.off();
        self.left.stop();
        self.right.stop();
        println!("stop");
    }
    pub fn speed(&mut self, left: f64, right: f64) {
        if left == 0.0 && right == 0.0 {
            self.stop();
        }
        else {
            let min_speed = 0.15;
            let left = (min_speed + left.abs() * (1.0 - min_speed)) * left.signum();
            let right = (min_speed + right.abs() * (1.0 - min_speed)) * right.signum();
            println!("speed {} {}", left, right);
            self.left.set_speed(left.abs());
            self.right.set_speed(right.abs());
            self.eep.on();
            if left > 0.0 {self.left.forward() } else {self.left.backward()};
            if right > 0.0 {self.right.forward() } else {self.right.backward()};
        }
    }
}
