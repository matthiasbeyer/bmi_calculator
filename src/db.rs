use std::io::{Read, Write};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseEntry {
    bmi: crate::bmi::BodyMassIndex,
    timestamp: time::PrimitiveDateTime,
}

impl DatabaseEntry {
    pub fn new(bmi: crate::bmi::BodyMassIndex) -> Result<Self, DatabaseError> {
        let timestamp = {
            let now = time::OffsetDateTime::now_local().map_err(DatabaseError::from)?;
            let date = now.date();
            let time = now.time();
            time::PrimitiveDateTime::new(date, time)
        };

        Ok(Self { bmi, timestamp })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Database(Vec<DatabaseEntry>);

impl Database {
    pub fn load() -> Result<Self, DatabaseError> {
        std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(false)
            .open("./database.json")
            .map_err(DatabaseError::from)
            .and_then(|mut file| {
                let mut buffer = String::new();
                let _ = file
                    .read_to_string(&mut buffer)
                    .map_err(DatabaseError::from)?;
                let database = serde_json::from_str(&buffer).map_err(DatabaseError::from)?;
                Ok(database)
            })
    }

    pub fn add_entry(&mut self, entry: DatabaseEntry) {
        self.0.push(entry);
    }

    pub fn store(self) -> Result<(), DatabaseError> {
        let bytes = serde_json::to_vec_pretty(&self)?;

        std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open("./database.json")
            .map_err(DatabaseError::from)
            .and_then(|mut file| file.write_all(&bytes).map_err(DatabaseError::from))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    IntermediateOffset(#[from] time::error::IndeterminateOffset),
}
