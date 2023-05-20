use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::Html,
    response::IntoResponse,
    Form,
};
use tokio::sync::RwLock;

use crate::db::Database;

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    entries: Vec<Entry>,
}

pub struct Entry {
    timestamp: String,
    bmi: String,
}

pub async fn index(State(database): State<Arc<RwLock<Database>>>) -> impl IntoResponse {
    let entries = database
        .read()
        .await
        .entry_iter()
        .map(|entry| Entry {
            timestamp: entry.timestamp().to_string(),
            bmi: entry.bmi().value().to_string(),
        })
        .collect();

    let template = IndexTemplate { entries };
    render_template(template)
}

#[derive(Template, Default)]
#[template(path = "form.html")]
pub struct FormTemplate {
    height: f64,
    weight: f64,
}

pub async fn new_entry() -> impl IntoResponse {
    render_template(FormTemplate::default())
}

#[derive(serde::Deserialize)]
pub struct FormFields {
    height: f64,
    weight: f64,
}

#[derive(Template)]
#[template(path = "newentry.html")]
pub struct NewEntry {
    bmi: f64,
}

#[derive(Template)]
#[template(path = "newentryerror.html")]
pub struct BmiError {
    error: String,
}

#[derive(Template)]
#[template(path = "newentryerror.html")]
pub struct DatabaseError {
    error: String,
}

pub async fn submit_new_entry(
    State(database): State<Arc<RwLock<Database>>>,
    Form(form): Form<FormFields>,
) -> impl IntoResponse {
    match crate::calculate_bmi(
        crate::weight::Weight(form.weight),
        crate::height::Height(form.height),
    ) {
        Ok(bmi) => match crate::db::DatabaseEntry::new(bmi.clone()) {
            Ok(entry) => {
                let mut write_handle = database.write().await;
                write_handle.add_entry(entry);
                if let Err(e) = write_handle.store() {
                    log::error!("{e:?}");
                } else {
                    log::info!("Database safe");
                }
                render_template(NewEntry { bmi: bmi.value() })
            }
            Err(dberror) => render_template(DatabaseError {
                error: format!("{dberror:?}"),
            }),
        },

        Err(error) => render_template(BmiError {
            error: format!("{error:?}"),
        }),
    }
}

fn render_template(template: impl Template) -> (StatusCode, Html<String>) {
    match template.render() {
        Ok(rendered) => {
            let code = StatusCode::OK;
            (code, Html(rendered))
        }
        Err(e) => {
            eprintln!("Failed to render template: {e:?}");

            (StatusCode::INTERNAL_SERVER_ERROR, Html(String::new()))
        }
    }
}
