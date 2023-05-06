use std::io::Write;
use std::str::FromStr;

struct Weight(f64);

struct Height(f64);

struct BodyMassIndex {
    value: f64,
}

// TODO: Eigene Datentypen für Eingabe und Ausgabe
fn calculate_bmi(weight: Weight, height: Height) -> Option<BodyMassIndex> {
    if height.0 <= 0.0 {
        return None;
    }
    let bmi = weight.0 / (height.0 * height.0);
    Some(BodyMassIndex { value: bmi })
}

#[test]
fn test_calculate_bmi() {
    let result = calculate_bmi(Weight(69.0), Height(1.69)).unwrap();
    assert_eq!(result.value, 24.158817968558527);
}

#[test]
fn test_calculate_bmi_broken() {
    let opt = calculate_bmi(Weight(69.0), Height(-0.0));
    assert!(opt.is_none());
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
        Some(bmi) => println!("BMI: {}", bmi.value),
        None => println!("Cannot use negative height"),
    }
}
