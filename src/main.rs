mod logic;
use clap::Parser;
use logic::*;
fn main() {
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
        )
        .unwrap();
        println!(">> Written example configuration to {p}.yml");
    }
    if let Some(b) = args.make {
        Cfg::new(b).run();
    }
}
