use std::io;
use std::io::Write;
use tconv::*;

enum Action {
    Convert(Mode),
    Help,
    Quit,
}

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
        let input: &str = &input.trim().to_lowercase();

        let action: Option<Action> = if input == "quit" {
            Some(Action::Quit)
        } else if input == "help" {
            Some(Action::Help)
        } else if input.ends_with("c") {
            Some(Action::Convert(Mode::ToFahrenheit))
        } else if input.ends_with("f") {
            Some(Action::Convert(Mode::ToCelsius))
        } else {
            None
        };

        match action {
            None => {
                print_unrecognized();
            }
            Some(action) => match action {
                Action::Quit => break,
                Action::Help => {
                    print_help();
                }
                Action::Convert(mode) => {
                    let input: &str = input[..input.len() - 1].trim();
                    let temperature: f64 = match input.parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    let temperature = match mode {
                        Mode::ToFahrenheit => Temperature::Celsius(temperature),
                        Mode::ToCelsius => Temperature::Fahrenheit(temperature),
                    };
                    let result = temperature.convert(mode);

                    print_result(&temperature, &result)
                }
            },
        };
    }
}
