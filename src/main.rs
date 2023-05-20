use axum::{
    routing::{get, post},
    Router, Server,
};
use bmi::BodyMassIndex;
use clap::Parser;
use error::BmiError;
use height::Height;
use inquire::CustomType;
use weight::Weight;

mod bmi;
mod db;
mod error;
mod height;
mod tests;
mod web;
mod weight;

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    database: bool,

    #[clap(short, long)]
    webui: bool,
}

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

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Program started!");

    let cli = Args::parse();
    if cli.database {
        println!("Printing database");
        let database = crate::db::Database::load().expect("Opening database");
        database.print();
        return;
    } else if cli.webui {
        println!("Starting webserver");
        let addr = "127.0.0.1:8123".parse().unwrap();
        let database = crate::db::Database::load().expect("Opening database");
        let database = std::sync::Arc::new(tokio::sync::RwLock::new(database));
        let router = Router::new()
            .route("/", get(web::index))
            .route("/new", get(web::new_entry))
            .route("/submit", post(web::submit_new_entry))
            .with_state(database);

        Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .unwrap();

        return;
    } else {
        println!("Interactive now...");
    }

    let weight = CustomType::<f64>::new("Gebe dein Gewicht in KG ein")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("Type the amount in Kilograms using a decimal point as a separator")
        .prompt()
        .map(Weight)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read weigth: {e:?}");
            std::process::exit(1)
        });

    log::debug!("Weight: {}", weight.0);

    let height = CustomType::<f64>::new("Gebe deine Größe in Meter ein")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("Type your height in meters")
        .prompt()
        .map(Height)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read heigth: {e:?}");
            std::process::exit(1)
        });

    println!("Weight: {}", height.0);

    let bmi = calculate_bmi(weight, height);
    match bmi {
        Ok(bmi) => {
            let value = BodyMassIndex::value(&bmi);
            println!("BMI: {}", value);

            let entry = db::DatabaseEntry::new(bmi).expect("Creating database entry object");
            let mut database = crate::db::Database::load().expect("Opening database");

            database.add_entry(entry);
            database.store().expect("Storing database");
        }
        Err(e) => println!("Error while calculating: {e:?}"),
    }
}
