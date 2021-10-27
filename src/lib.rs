use math::round;

#[derive(PartialEq, Debug)]
pub enum Mode {
    ToFahrenheit,
    ToCelsius,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    Convert(Temperature, Mode),
    Help,
    Quit,
    Unrecognized,
}

#[derive(Debug)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Action {
    pub fn parse(input: &str) -> Self {
        let input: &str = &input.trim().to_lowercase();

        if input == "quit" {
            Action::Quit
        } else if input == "help" {
            Action::Help
        } else if input.ends_with("c") || input.ends_with("f") {
            let value: &str = input[..input.len() - 1].trim();
            let value: f64 = value.parse().unwrap();
            let temperature = if input.ends_with("c") {
                Temperature::Celsius(value)
            } else {
                Temperature::Fahrenheit(value)
            };
            match temperature {
                Temperature::Celsius(_) => Action::Convert(temperature, Mode::ToFahrenheit),
                Temperature::Fahrenheit(_) => Action::Convert(temperature, Mode::ToCelsius),
            }
        } else {
            Action::Unrecognized
        }
    }
}

impl Temperature {
    pub fn convert(&self, mode: Mode) -> Temperature {
        match mode {
            Mode::ToFahrenheit => self.to_fahrenheit(),
            Mode::ToCelsius => self.to_celsius(),
        }
    }

    fn to_fahrenheit(&self) -> Temperature {
        match self {
            Temperature::Celsius(value) => Temperature::Fahrenheit(*value * 1.8 + 32.0),
            Temperature::Fahrenheit(value) => Temperature::Fahrenheit(*value),
        }
    }

    fn to_celsius(&self) -> Temperature {
        match self {
            Temperature::Celsius(value) => Temperature::Celsius(*value),
            Temperature::Fahrenheit(value) => Temperature::Celsius((*value - 32.0) / 1.8),
        }
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        use Temperature::*;
        match (self, other) {
            (Celsius(a), Celsius(b)) => round::half_up(*a, 2) == round::half_up(*b, 2),
            (Fahrenheit(a), Fahrenheit(b)) => round::half_up(*a, 2) == round::half_up(*b, 2),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_celsius_conversion() {
        let temperature = Temperature::Celsius(0.1);
        assert_eq!(temperature, temperature.to_celsius());
    }

    #[test]
    fn fahrenheit_to_fahrenheit_conversion() {
        let temperature = Temperature::Fahrenheit(0.1);
        assert_eq!(temperature, temperature.to_fahrenheit());
    }

    #[test]
    fn celsius_0_to_fahrenheit_conversion() {
        let temperature = Temperature::Celsius(0.0);
        assert_eq!(Temperature::Fahrenheit(32.0), temperature.to_fahrenheit());
    }

    #[test]
    fn fahrenheit_0_to_celsius_conversion() {
        let temperature = Temperature::Fahrenheit(0.0);
        assert_eq!(Temperature::Celsius(-17.78), temperature.to_celsius());
    }

    #[test]
    fn fahrenheit_451_to_celsius_conversion() {
        let temperature = Temperature::Fahrenheit(451.0);
        assert_eq!(Temperature::Celsius(232.78), temperature.to_celsius());
    }

    #[test]
    fn parse_help_action() {
        let result = Action::parse("help");
        assert_eq!(Action::Help, result);

        let result = Action::parse(" HeLp  ");
        assert_eq!(Action::Help, result);
    }

    #[test]
    fn parse_quit_action() {
        let result = Action::parse("quit");
        assert_eq!(Action::Quit, result);

        let result = Action::parse("  qUiT  ");
        assert_eq!(Action::Quit, result);
    }

    #[test]
    fn parse_unrecognized_action() {
        let result = Action::parse("something");
        assert_eq!(Action::Unrecognized, result);
    }

    #[test]
    fn parse_convert_celsius_action() {
        let result = Action::parse(" 10c ");
        assert_eq!(
            Action::Convert(Temperature::Celsius(10.0), Mode::ToFahrenheit),
            result
        );

        let result = Action::parse(" 10.0C ");
        assert_eq!(
            Action::Convert(Temperature::Celsius(10.0), Mode::ToFahrenheit),
            result
        );
    }

    #[test]
    fn parse_convert_fahrenheit_action() {
        let result = Action::parse(" 10f ");
        assert_eq!(
            Action::Convert(Temperature::Fahrenheit(10.0), Mode::ToCelsius),
            result
        );

        let result = Action::parse(" 10.0F ");
        assert_eq!(
            Action::Convert(Temperature::Fahrenheit(10.0), Mode::ToCelsius),
            result
        );
    }

    #[test]
    #[should_panic]
    fn panics_for_not_parsable_temperature() {
        Action::parse("10cf");
    }
}
