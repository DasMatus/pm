use filedb::DB;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Debug)]
struct Data {
    name: String,
    surname: String,
}
fn main() {
    let mut db: DB<Data> = DB::new("example".to_string());
    db.populate(
        "government".to_string(),
        "id".to_string(),
        Data {
            name: "".to_string(),
            surname: "".to_string(),
        },
    );
    println!("{:#?}", db.open("government".to_string(), "id".to_string()));
}
