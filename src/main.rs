use std::io::Write;
use std::str::FromStr;

use bmi::BodyMassIndex;
use error::BmiError;
use height::Height;
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
    let stdin = std::io::stdin();
    print!("Gebe dein Gewicht in kg ein: ");
    let _ = std::io::stdout().flush();

    // TODO Error handling:
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    println!(); // newline

    // TODO Error handling:
    let weight = Weight(f64::from_str(buffer.trim()).unwrap());
    println!("Weight: {}", weight.0);

    print!("Gebe deine Größe in Meter ein: ");
    let _ = std::io::stdout().flush();

    let mut buffer_height = String::new();
    // TODO Error handling:
    stdin.read_line(&mut buffer_height).unwrap();
    println!(); // newline

    // TODO Error handling:
    let height = Height(f64::from_str(buffer_height.trim()).unwrap());
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
