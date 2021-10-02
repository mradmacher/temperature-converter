pub enum Mode {
    ToFahrenheit,
    ToCelsius
}

pub enum Action {
    Convert(Mode),
    Help,
    Quit,
}

#[derive(Debug)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

pub fn convert(mode: Mode, temperature: &Temperature) -> Temperature {
    match mode {
        Mode::ToFahrenheit => to_fahrenheit(&temperature),
        Mode::ToCelsius => to_celsius(&temperature),
    }
}

pub fn to_fahrenheit(temperature: &Temperature) -> Temperature {
    match temperature {
        Temperature::Celsius(value) => Temperature::Fahrenheit(*value * 1.8 + 32.0),
        Temperature::Fahrenheit(value) => Temperature::Fahrenheit(*value),
    }
}

pub fn to_celsius(temperature: &Temperature) -> Temperature {
    match temperature {
        Temperature::Celsius(value) => Temperature::Celsius(*value),
        Temperature::Fahrenheit(value) => Temperature::Celsius((*value - 32.0)/1.8),
    }
}
