use filedb::DB;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Data {
    name: String,
    surname: String,
}
fn main() {
    // Initialize the database
    let mut db: DB<Data> = DB::new("database".to_string());
    db.populate("id".to_string(), "personal".to_string(), Data::default());
}
