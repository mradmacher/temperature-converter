use std::io;
use std::io::Write;

enum Mode {
    ToFahrenheit,
    ToCelsius
}

enum Action {
    Convert(Mode),
    Help,
    Quit,
}

#[derive(Debug)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

fn convert(mode: Mode, temperature: &Temperature) -> Temperature {
    match mode {
        Mode::ToFahrenheit => to_fahrenheit(&temperature),
        Mode::ToCelsius => to_celsius(&temperature),
    }
}

fn to_fahrenheit(temperature: &Temperature) -> Temperature {
    match temperature {
        Temperature::Celsius(value) => Temperature::Fahrenheit(*value * 1.8 + 32.0),
        Temperature::Fahrenheit(value) => Temperature::Fahrenheit(*value),
    }
}

fn to_celsius(temperature: &Temperature) -> Temperature {
    match temperature {
        Temperature::Celsius(value) => Temperature::Celsius(*value),
        Temperature::Fahrenheit(value) => Temperature::Celsius((*value - 32.0)/1.8),
    }
}

fn main() {
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
                println!("Unrecognized input. Type `help` to get some hints.");
            },
            Some(action) => {
                match action {
                    Action::Quit => break,
                    Action::Help => {
                        println!("Enter temperature with a unit (C for Celsius, F for Fahrenheit).");
                        println!("Type `help` to display this message");
                        println!("Type `quit` to leave the program.");
                    },
                    Action::Convert(mode) => {
                        let input: &str = input[..input.len()-1].trim();
                        let temperature: f64 = match input.parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        };

                        let temperature = match mode {
                            Mode::ToFahrenheit => Temperature::Celsius(temperature),
                            Mode::ToCelsius => Temperature::Fahrenheit(temperature),
                        };
                        let result = convert(mode, &temperature);

                        println!("{:?} => {:?}", temperature, result);
                    }
                }
            }
        };
    }
}
