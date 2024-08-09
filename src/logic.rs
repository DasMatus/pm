use clap::Parser;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub(crate) struct Cfg {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) dependencies: Vec<String>,
    pub(crate) provides: Vec<String>,
    pub(crate) prepare: Vec<Prepare>,
    pub(crate) build: Vec<Build>,
    pub(crate) install: Vec<Install>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Prepare {
    pub(crate) step: String,
    pub(crate) command: Vec<String>,
    pub(crate) chdir: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Build {
    pub(crate) step: String,
    pub(crate) command: Vec<String>,
    pub(crate) chdir: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Install {
    pub(crate) step: String,
    pub(crate) command: Vec<String>,
    pub(crate) chdir: Option<String>,
}
#[derive(Parser)]
#[clap(name = "pm", about, long_about = None)]
pub(crate) struct Cli {
    #[arg(long = "init")]
    pub(crate) init: Option<String>,
    #[arg(long = "build")]
    pub(crate) make: Option<String>,
}
impl Cfg {
    pub(crate) fn new(c: String) -> Self {
        let yaml: Self = serde_yaml::from_str(
            std::fs::read_to_string(c.as_str()).unwrap().as_str(),
        )
        .unwrap();
        Self {
            name: yaml.name,
            version: yaml.version,
            dependencies: yaml.dependencies,
            provides: yaml.provides,
            prepare: yaml.prepare,
            build: yaml.build,
            install: yaml.install,
        }
    }
    pub(crate) fn run(self) {
        let len: usize =
            self.dependencies.len() + 
            self.prepare.len()      + 
            self.build.len()        + 
            self.install.len();
        let bar =
            indicatif::ProgressBar::new((len as usize).try_into().unwrap());
        bar.style();
        println!(">> ");
    }
}
