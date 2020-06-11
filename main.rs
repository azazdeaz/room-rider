use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use wheels::Wheels;

fn main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();

    let mut wheels = Wheels::new();

    // let mut count = 0;

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        //i reckon this speaks for itself
        let key = c.unwrap();
        println!("pressed {:?}", key);
        match key {
            Key::Up => wheels.forward(),
            Key::Down => wheels.backward(),
            Key::Left => wheels.left(),
            Key::Right => wheels.right(),
            Key::Char(' ') | Key::Char('\n') => wheels.stop(),
            Key::Ctrl('q') | Key::Ctrl('c') => break,
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}