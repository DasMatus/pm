#![warn(clippy::all)]
mod logic;
use std::path::Path;

use clap::Parser;
use filedb_ng::DB;
use logic::*;
fn main() -> anyhow::Result<(), anyhow::Error> {
    println!(">> Checking if the database exists");
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let db_loc = &Path::new(&std::env::home_dir().unwrap())
        .join("mtos/filedb")
        .to_str()
        .unwrap()
        .to_string();
    #[cfg(target_os = "linux")]
    let db_loc = "/mtos/filedb";
    if Path::new(db_loc).exists() {
        println!("\t>> Database exists, continuing");
    } else {
        println!("\t>> Creating database at {db_loc}");
        let mut db: DB<Contents> = DB::new(db_loc.to_string())?;
        db.populate("pkgs".to_string(), "db".to_string(), None)?;
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
            dir: None,
        }],
        build: vec![Build {
            step: "".to_string(),
            command: vec!["".to_string()],
            chdir: Some("".to_string()),
        }],
        install: vec![Build {
            step: "".to_string(),
            command: vec!["".to_string()],
            chdir: Some("".to_string()),
        }],
    };
    if let Some(p) = args.path {
        std::fs::write(
            format!("{p}.yml"),
            format!("---\n{}", serde_yaml::to_string(&cfg).unwrap()),
        )?;
        println!(">> Written example configuration to {p}.yml");
    }
    if let Some(b) = args.file {
        let db: DB<Contents> = DB::new(db_loc.to_string())?;
        let cfg: Contents = db.open("pkgs".to_string(), "db".to_string())?;
        for i in cfg.pkgs {
            if b == i {
                println!(">> Package {i} exists, not rebuilding it");
                continue;
            } else {
                Cfg::new(b.clone())?.run()?;
            }
        }
    }
    Ok(())
}
