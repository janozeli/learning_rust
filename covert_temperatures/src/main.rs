use core::str::FromStr;
use std::io;

enum TemperatureScale {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl FromStr for TemperatureScale {
    type Err = ();

    fn from_str(input: &str) -> Result<TemperatureScale, Self::Err> {
        match input {
            "1" => Ok(TemperatureScale::Celsius),
            "2" => Ok(TemperatureScale::Fahrenheit),
            "3" => Ok(TemperatureScale::Kelvin),
            _ => Err(()),
        }
    }
}

fn convert_temperature(from: TemperatureScale, to: TemperatureScale, value: f32) -> f32 {
    match (from, to) {
        (TemperatureScale::Celsius, TemperatureScale::Fahrenheit) => value * 1.8 + 32.0,
        (TemperatureScale::Fahrenheit, TemperatureScale::Celsius) => (value - 32.0) / 1.8,
        (TemperatureScale::Celsius, TemperatureScale::Kelvin) => value + 273.15,
        (TemperatureScale::Kelvin, TemperatureScale::Celsius) => value - 273.15,
        (TemperatureScale::Fahrenheit, TemperatureScale::Kelvin) => {
            (value - 32.0) * 5.0 / 9.0 + 273.15
        }
        (TemperatureScale::Kelvin, TemperatureScale::Fahrenheit) => {
            (value - 273.15) * 9.0 / 5.0 + 32.0
        }
        _ => value,
    }
}

fn main() {
    let mut original_temperature_scale = String::new();
    let mut original_temperature = String::new();
    let mut target_temperature_scale = String::new();

    println!("--- Converting Temperatures ---");

    println!("Select the original temperature unit: ");
    println!("(1) Celsius (2) Fanrenheit (3) Kelvin");

    io::stdin()
        .read_line(&mut original_temperature_scale)
        .unwrap();

    let original_temperature_scale: TemperatureScale =
        original_temperature_scale.trim().parse().unwrap();

    println!("Insert the temperature value: ");
    io::stdin().read_line(&mut original_temperature).unwrap();
    let original_temperature: f32 = original_temperature.trim().parse().unwrap();

    println!("Select the target temperature unit: ");
    println!("(1) Celsius (2) Fanrenheit (3) Kelvin");

    io::stdin()
        .read_line(&mut target_temperature_scale)
        .unwrap();

    let target_temperature_scale: TemperatureScale =
        target_temperature_scale.trim().parse().unwrap();

    let target_temperature: f32 = convert_temperature(
        original_temperature_scale,
        target_temperature_scale,
        original_temperature,
    );

    println!("The result is {target_temperature}");
}
