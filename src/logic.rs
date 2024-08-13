use std::process::Command;

use clap::Parser;
use derive_more::AsRef;
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
    pub(crate) install: Vec<Build>,
}

#[derive(Serialize, Deserialize, Default, AsRef)]
pub(crate) struct Contents {
    #[as_ref(forward)]
    pub(crate) pkgs: Vec<String>,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct DL {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) ft: String,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct Prepare {
    pub(crate) dl: Option<Vec<DL>>,
    pub(crate) step: String,
    pub(crate) dir: Option<Vec<String>>,
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
    /// Where you want your file to be
    pub(crate) path: Option<String>,
    #[arg(long = "build")]
    /// Path to the build file
    pub(crate) file: Option<String>,
}
impl Cfg {
    pub(crate) fn new(c: String) -> Self {
        let yaml: Self = serde_yaml::from_str(
            std::fs::read_to_string(c.as_str())?().as_str(),
        )
        ?();
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
    pub(crate) fn run(self) -> anyhow::Result<(), anyhow::Error> {
        let len: usize = self.dependencies.len()
            + self.dependencies.len()
            + self.prepare.len()
            + self.build.len()
            + self.install.len();
        let bar = indicatif::ProgressBar::new((len as usize).try_into()?);
        bar.set_style(indicatif::ProgressStyle::default_bar());
        bar.set_message(format!("Making package {}", self.name));
        let cesta = std::path::Path::new("/tmp").join(self.name);
        // prepare part
        for i in self.prepare {
            if let Some(i) = i.dl {
                for url in i {
                    bar.set_message(format!(
                        "Downloading {:#?} into {}/{}",
                        cesta.as_path().to_str(),
                        url.url,
                        format!("{}{}", url.name, url.ft)
                    ));
                    std::fs::DirBuilder::new()
                        .recursive(true)
                        .create(&cesta)?;
                    fetch_data::download(url.url, &cesta)?;
                    for file in std::fs::read_dir(&cesta)? {
                        let d = file?;
                        compress_tools::uncompress_archive(
                            d,
                            Path::new("source").join(d.file_name()),
                            compress_tools::Ownership::Preserve,
                        );
                        bar.set_message(format!(
                            "Decompressing {d} to {}",
                            Path::new("source").join(d.file_name())
                        ));
                    }
                }
            }
            bar.set_message(format!("Running prepare task {}", i.step));
            std::env::set_current_dir("source");
            if let Some(chdir) = i.chdir {
                Command::new(i.command[1])
                    .args(i.prepare[2..i.command.len()])
                    .current_dir(chdir)
                    .status()?
            }
            Command::new(i.command[1])
                .args(i.prepare[2..i.command.len()])
                .status()?
        }
        // build
        for build in self.build {
            bar.set_message(format!("Running command {}", build.command.iter()));
            if let Some(chdir) = i.chdir {
                Command::new(build.command[1])
                    .args(build.command[2..=build.command.len()].iter())
                    .current_dir(chdir)
                    .status()?
            }
            Command::new(build.command[1])
                .args(build.command[2..=build.command.len()].iter())
                .status()?
        }
        // install
        for install in self.install {
            bar.set_message(format!("Running command {}", build.command.iter()));
            if let Some(chdir) = i.chdir {
                Command::new(install.command[1])
                    .args(install.command[2..=build.command.len()].iter())
                    .current_dir(chdir)
                    .status()?
            }
            Command::new(install.command[1])
                .args(install.command[2..=install.command.len()].iter())
                .status()?
        }
        Ok(())
    }
}