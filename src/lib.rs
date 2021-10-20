pub enum Mode {
    ToFahrenheit,
    ToCelsius,
}

#[derive(PartialEq, Debug)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_celsius_conversion() {
        let temperature = Temperature::Celsius(0.0);
        assert_eq!(temperature, temperature.to_celsius());
    }

    #[test]
    fn fahrenheit_to_fahrenheit_conversion() {
        let temperature = Temperature::Fahrenheit(0.0);
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
}
