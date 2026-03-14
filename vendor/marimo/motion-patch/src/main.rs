#[cfg(feature = "cli")]
use std::{collections::BTreeMap, path::PathBuf};

#[cfg(feature = "cli")]
use hash40::Hash40;

#[cfg(feature = "cli")]
use motion_patch::{MotionMapExt, MotionPatch};

#[cfg(not(feature = "cli"))]
compile_error!("Specify the 'cli' feature to build the differ");

#[cfg(feature = "cli")]
#[derive(clap::Parser, Debug)]
pub enum Args {
    Diff {
        #[clap(long)]
        labels: PathBuf,

        #[clap(long)]
        source: PathBuf,

        #[clap(long)]
        target: PathBuf,

        #[clap(long)]
        output: PathBuf,
    },
    Apply {
        #[clap(long)]
        source: PathBuf,

        #[clap(long)]
        patch: PathBuf,

        #[clap(long)]
        output: PathBuf,
    },
    Sort {
        #[clap(long)]
        path: PathBuf,
    },
}

#[cfg(feature = "cli")]
impl Args {
    pub fn perform(self) {
        match self {
            Self::Diff {
                labels,
                source,
                target,
                output,
            } => {
                if Hash40::label_map()
                    .lock()
                    .unwrap()
                    .add_labels_from_path(labels)
                    .is_err()
                {
                    eprintln!("Pass a valid labels file");
                    return;
                }

                let Ok(src) = motion_lib::open(source) else {
                    eprintln!("Pass a valid source motion_list.bin file");
                    return;
                };

                let Ok(dst) = motion_lib::open(target) else {
                    eprintln!("Pass a valid target motion_list.bin file");
                    return;
                };

                let diff = BTreeMap::create(&src, &dst);
                let string = serde_yaml::to_string(&diff).unwrap();
                if let Ok(resorted) =
                    serde_yaml::from_str::<BTreeMap<String, serde_yaml::Value>>(&string)
                {
                    let string = serde_yaml::to_string(&resorted).unwrap();
                    if std::fs::write(output, &string).is_err() {
                        eprintln!("Failed to write patch file!");
                        eprintln!("{string}");
                    }
                } else if std::fs::write(output, &string).is_err() {
                    eprintln!("Failed to write patch file!");
                    eprintln!("{string}");
                }
            }
            Self::Apply {
                source,
                patch,
                output,
            } => {
                let Ok(mut src) = motion_lib::open(source) else {
                    eprintln!("Pass a valid source motion_list.bin file");
                    return;
                };

                let Ok(patch_str) = std::fs::read_to_string(patch) else {
                    eprintln!("Pass a valid patch file");
                    return;
                };

                let Ok(patch) = serde_yaml::from_str::<BTreeMap<Hash40, MotionPatch>>(&patch_str)
                else {
                    eprintln!("The patch file was not valid yaml!");
                    return;
                };

                patch.apply(&mut src);

                if motion_lib::save(output, &src).is_err() {
                    eprintln!("Failed to write new motion file!");
                }
            }
            Self::Sort { path } => {
                let Ok(str) = std::fs::read_to_string(&path) else {
                    eprintln!("File was not valid");
                    return;
                };

                if let Ok(resorted) =
                    serde_yaml::from_str::<BTreeMap<String, serde_yaml::Value>>(&str)
                {
                    let string = serde_yaml::to_string(&resorted).unwrap();
                    if std::fs::write(path, &string).is_err() {
                        eprintln!("Failed to write patch file!");
                        eprintln!("{string}");
                    }
                }
            }
        }
    }
}

fn main() {
    #[cfg(feature = "cli")]
    {
        use clap::Parser;
        Args::parse().perform();
    }
}
