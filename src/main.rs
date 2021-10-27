use std::io;
use std::io::Write;
use tconv::*;

fn print_help() {
    println!("Enter temperature with a unit (C for Celsius, F for Fahrenheit).");
    println!("Type `help` to display this message");
    println!("Type `quit` to leave the program.");
}

fn print_unrecognized() {
    println!("Unrecognized input. Type `help` to get some hints.");
}

fn print_result(temperature: &Temperature, result: &Temperature) {
    println!("{:?} => {:?}", temperature, result);
}

fn main() {
    print_help();

    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let action = Action::parse(&input);

        match action {
            Action::Unrecognized => print_unrecognized(),
            Action::Quit => break,
            Action::Help => print_help(),
            Action::Convert(temperature, mode) => {
                print_result(&temperature, &temperature.convert(mode));
            }
        };
    }
}
