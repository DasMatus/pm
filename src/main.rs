mod logic;
use std::path::Path;

use clap::Parser;
use filedb::DB;
use logic::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Contents {
    pkgs: Vec<String>,
}
fn main() -> anyhow::Result<(), anyhow::Error> {
    println!(">> Checking if the database exists");
    let db_loc = "/mtos/filedb";
    if Path::new(db_loc).exists() {
        println!("\t>> Database exists, continuing");
    } else {
        let mut db: DB<Contents> = DB::new(db_loc.to_string());
        db.populate("pkgs".to_string(), "db".to_string(), None);
    }
    let args = Cli::parse();
    let cfg = Cfg {
        name: "".to_string(),
        version: "".to_string(),
        dependencies: vec!["".to_string()],
        provides: vec!["".to_string()],
        prepare: vec![Prepare {
            step: "".to_string(),
            command: vec!["".to_string()],
            chdir: Some("".to_string()),
            dl: Some(vec![DL {
                url: "".to_string(),
                name: "".to_string(),
                ft: "".to_string(),
            }]),
        }],
        build: vec![Build {
            step: "".to_string(),
            command: vec!["".to_string()],
            chdir: Some("".to_string()),
        }],
        install: vec![Install {
            step: "".to_string(),
            command: vec!["".to_string()],
            chdir: Some("".to_string()),
        }],
    };
    if let Some(p) = args.init {
        std::fs::write(
            format!("{p}.yml"),
            format!("---\n{}", serde_yaml::to_string(&cfg).unwrap()),
        )?;
        println!(">> Written example configuration to {p}.yml");
    }
    if let Some(b) = args.make {
        Cfg::new(b).run()?;
    }
    Ok(())
}
