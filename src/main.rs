use std::str::FromStr;
use std::io::Write;

// TODO: Eigene Datentypen für Eingabe und Ausgabe
fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
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
    let weight = f64::from_str(buffer.trim()).unwrap();
    println!("Weight: {weight}");


    print!("Gebe deine Größe in Meter ein: ");
    let _ = std::io::stdout().flush();

    let mut buffer_height = String::new();
    // TODO Error handling:
    stdin.read_line(&mut buffer_height).unwrap();
    println!(""); // newline

    // TODO Error handling:
    let height = f64::from_str(buffer_height.trim()).unwrap();
    println!("Weight: {height}");

    let bmi = calculate_bmi(weight, height);
    println!("BMI: {bmi}");
}
