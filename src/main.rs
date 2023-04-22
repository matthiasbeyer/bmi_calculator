use std::io::Write;
use std::str::FromStr;

struct Weight(f64);

struct Height(f64);

struct BMI {
    value: f64,
}

// TODO: Eigene Datentypen für Eingabe und Ausgabe
fn calculate_bmi(weight: Weight, height: Height) -> BMI {
    let bmi = weight.0 / (height.0 * height.0);
    BMI { value: bmi }
}

fn main() {
    let stdin = std::io::stdin();
    print!("Gebe dein Gewicht in kg ein: ");
    let _ = std::io::stdout().flush();

    // TODO Error handling:
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    println!(""); // newline

    // TODO Error handling:
    let weight = Weight(f64::from_str(buffer.trim()).unwrap());
    println!("Weight: {}", weight.0);

    print!("Gebe deine Größe in Meter ein: ");
    let _ = std::io::stdout().flush();

    let mut buffer_height = String::new();
    // TODO Error handling:
    stdin.read_line(&mut buffer_height).unwrap();
    println!(""); // newline

    // TODO Error handling:
    let height = Height(f64::from_str(buffer_height.trim()).unwrap());
    println!("Weight: {}", height.0);

    let bmi = calculate_bmi(weight, height);
    println!("BMI: {}", bmi.value);
}
