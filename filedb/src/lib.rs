//! # `filedb`
//! A **library-only** database that uses files instead of single monolith `.db` file.
//! It's very simple, since it only handles writing and reading to/from files and doesn't include encryption and CLI.
//! > You have to do these things yourself, either through having the database on something like [VeraCrypt volume](https://veracrypt.org) or on another machine.
//! > Anyway, take a look around.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
/// Main struct that handles the database
#[derive(Default, Serialize, Deserialize)]
pub struct DB<Struct: Serialize> {
    // row
    db_row: String,
    // location of the database
    db_location: String,
    table: String, // aka directory
    // contents
    contents: Option<Struct>,
}
impl<Struct: Serialize + Default + for<'de> serde::Deserialize<'de>>
    DB<Struct>
{
    /// Initializes the database
    pub fn new(location: String) -> Result<Self, anyhow::Error> {
        if !Path::new(&location).exists() {
            std::fs::DirBuilder::new()
                .recursive(true)
                .create(&location)
                .unwrap()
        }
        Ok(Self {
            db_row: "".to_string(),
            db_location: location,
            table: "".to_string(),
            contents: Default::default(),
        })
    }
    /// Populates the database with content
    pub fn populate(
        &mut self,
        table: String,
        row: String,
        contents: Option<Struct>,
    ) -> Result<(), anyhow::Error> {
        let join = Path::new(&self.db_location).join(&table);
        match join.exists() {
            true => (),
            false => std::fs::DirBuilder::new()
                .recursive(true)
                .create(&join)
                .unwrap(),
        };
        self.table = table;
        self.contents = contents;
        self.db_row = row;
        let toml = toml::to_string_pretty(&self.contents);
        std::fs::write(
            Path::new(join.to_str().unwrap()).join(&self.db_row),
            toml?,
        )?;
        Ok(())
    }
    /// Opens the database
    pub fn open(
        self,
        table: String,
        row: String,
    ) -> Result<Struct, anyhow::Error> {
        let path = Path::new(&self.db_location).join(table).join(row);
        let f = std::fs::read_to_string(&path)?;
        Ok(toml::from_str::<Struct>(f.as_str())?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default)]
    struct Data {
        name: String,
        surname: String,
    }
    #[test]
    fn test_create_db() -> anyhow::Result<(), anyhow::Error> {
        // Initialize the database
        let mut db: DB<Data> = DB::new("database".to_string())?;
        db.populate("id".to_string(), "personal".to_string(), None)?;
        std::fs::remove_dir_all("database").unwrap();
        Ok(())
    }
}
