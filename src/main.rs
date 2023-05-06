use bmi::BodyMassIndex;
use error::BmiError;
use height::Height;
use inquire::CustomType;
use weight::Weight;

mod bmi;
mod error;
mod height;
mod tests;
mod weight;

// TODO: Eigene Datentypen für Eingabe und Ausgabe
pub fn calculate_bmi(weight: Weight, height: Height) -> Result<BodyMassIndex, BmiError> {
    if height.0 <= 0.0 {
        return Err(BmiError::HeightCannotBeZeroOrSmaller);
    }

    if weight.0 <= 0.0 {
        return Err(BmiError::WeightCannotBeZeroOrSmaller);
    }

    let bmi = weight.0 / (height.0 * height.0);
    Ok(BodyMassIndex::new(bmi))
}

fn main() {
    env_logger::init();

    log::info!("Program started!");

    let weight = CustomType::<f64>::new("Gebe dein Gewicht in KG ein")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("Type the amount in Kilograms using a decimal point as a separator")
        .prompt();

    let weight = Weight(weight.unwrap());
    log::debug!("Weight: {}", weight.0);

    let height = CustomType::<f64>::new("Gebe deine Größe in Meter ein")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("Type your height in meters")
        .prompt();

    let height = Height(height.unwrap());
    println!("Weight: {}", height.0);

    let bmi = calculate_bmi(weight, height);
    match bmi {
        Ok(bmi) => {
            let value = BodyMassIndex::value(&bmi);
            println!("BMI: {}", value);
        }
        Err(e) => println!("Error while calculating: {e:?}"),
    }
}
