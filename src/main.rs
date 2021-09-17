use std::io;

enum Mode {
    ToFahrenheit,
    ToCelsius
}

enum Action {
    Convert(Mode),
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
        let mut action = String::new();

        println!("Select action:");
        println!("1. Convert from celsius to fahrenheit");
        println!("2. Convert from fahrenheit to celsius");
        println!("3. Quit");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        let action: &str = action.trim();

        let action: Option<Action> = if action == "1" {
            Some(Action::Convert(Mode::ToFahrenheit))
        } else if action == "2" {
            Some(Action::Convert(Mode::ToCelsius))
        } else if action == "3" {
            Some(Action::Quit)
        } else {
            None
        };

        match action {
            None => {
                println!("Unknown action");
            },
            Some(action) => {
                match action {
                    Action::Quit => break,
                    Action::Convert(mode) => {
                        let mut input = String::new();

                        println!("Enter temperature:");
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let input: &str = input.trim();

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
                        println!("----------");
                    }
                }
            }
        };
    }
}
