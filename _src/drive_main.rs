extern crate panic_abort;
extern crate cortex_m;
extern crate embedded_hal;
extern crate tm4c123x_hal as hal;
extern crate stepper_rs;

use hal::delay::Delay;
use hal::gpio::GpioExt;
use hal::sysctl::SysctlExt;

fn main() {
    let p = hal::Peripherals::take().unwrap();
    let sysctl = p.SYSCTL.constrain();
    let portc = p.GPIO_PORTC.split(&sysctl.power_control);
    let clocks = sysctl.clock_setup.freeze();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut driver = stepper_rs::MotorDriver::a4988(
        Delay::new(cp.SYST, &clocks),
        portc.pc6.into_push_pull_output(),
        portc.pc7.into_push_pull_output(),
        200,
        16,
        100f32
    );

    loop {
        driver.set_speed(100f32);
        driver.set_direction(true);
        driver.move_instant(600);
        driver.set_direction(false);
        driver.move_instant(600);

        driver.set_speed(300f32);
        driver.set_direction(true);
        driver.move_smooth(1600, 150, 150);
        driver.set_direction(false);
        driver.move_smooth(1600, 150, 150);
    }
}