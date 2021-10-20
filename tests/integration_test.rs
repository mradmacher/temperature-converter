use tconv;

#[test]
fn celsius_to_celsius_conversion() {
    let temperature = tconv::Temperature::Celsius(0.0);
    assert_eq!(temperature.convert(tconv::Mode::ToCelsius), temperature);
}

#[test]
fn fahrenheit_to_fahrenheit_conversion() {
    let temperature = tconv::Temperature::Fahrenheit(0.0);
    assert_eq!(temperature.convert(tconv::Mode::ToFahrenheit), temperature);
}
