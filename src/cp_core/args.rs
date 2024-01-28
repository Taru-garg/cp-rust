use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct Args {
    pub source: Vec<PathBuf>,
    pub target: Vec<PathBuf>,
    options: Option<CopyOptions>,
}

impl Args {
    fn path_exists(path: &String) -> bool {
        return std::path::Path::new(path).exists();
    }

    fn check_paths(source: &String, target: &String) -> Result<(), String> {
        if !Args::path_exists(source) {
            return Err(String::from("Source Path doesn't exist"));
        }

        if Args::path_exists(target) {
            return Err(String::from("Target Path already exists"));
        }

        return Ok(());
    }

    fn extract_copy_sources(path: &PathBuf) -> Vec<PathBuf> {
        if path.is_dir() {
            return std::fs::read_dir(path)
                .expect(
                    format!(
                        "Unable to read the directory at path : {}",
                        path.to_str().unwrap()
                    )
                    .as_str(),
                )
                .into_iter()
                .map(|x| {
                    if x.as_ref().unwrap().path().is_dir() == true {
                        return Args::extract_copy_sources(&x.unwrap().path());
                    } else {
                        return vec![x.as_ref().unwrap().path()];
                    }
                })
                .flatten()
                .collect();
        }
        return vec![path.to_path_buf()];
    }

    fn extract_copy_targets(
        sources: &Vec<PathBuf>,
        initial_source: &PathBuf,
        target: &PathBuf,
    ) -> Vec<PathBuf> {
        return sources
            .into_iter()
            .map(|x| {
                if initial_source.is_dir() {
                    let rel = x
                        .strip_prefix(initial_source)
                        .expect("Invalid source path found in files resolved for copying");
                    return target.as_path().join(rel).to_path_buf();
                }
                return PathBuf::from(target);
            })
            .collect::<Vec<_>>();
    }

    pub fn parser() -> Result<Self, String> {
        let source_path = env::args().nth(1).expect("No Source path provided!");
        let target_path = env::args().nth(2).expect("No Target path provided!");

        if Args::check_paths(&source_path, &target_path).is_ok() {
            let source_files = Args::extract_copy_sources(&PathBuf::from(&source_path));
            let target_files = Args::extract_copy_targets(
                &source_files,
                &PathBuf::from(&source_path),
                &PathBuf::from(&target_path),
            );
            return Ok(Args {
                source: source_files,
                target: target_files,
                options: None,
            });
        }
        return Err(String::from("Targets are not valid for copy."));
    }
}

#[derive(Debug)]
// TODO: implement various copy options
pub struct CopyOptions {}
