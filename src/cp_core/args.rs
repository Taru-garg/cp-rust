use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Args {
    source: Vec<PathBuf>,
    target: String,
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

    fn extract_copy_tagets(path: &Path) -> Vec<PathBuf> {
        if path.is_dir() {
            let data: Vec<PathBuf> = std::fs::read_dir(path)
                .unwrap()
                .into_iter()
                .map(|x| {
                    if x.as_ref().unwrap().path().is_dir() == true {
                        return Args::extract_copy_tagets(x.as_ref().unwrap().path().as_path());
                    } else {
                        return vec![x.as_ref().unwrap().path()];
                    }
                })
                .flatten()
                .collect();

            return data;
        }
        return vec![path.to_path_buf()];
    }
    pub fn parser() -> Result<Self, String> {
        let source_path = env::args().nth(1).expect("No Source path provided!");
        let target_path = env::args().nth(2).expect("No Target path provided!");

        if Args::check_paths(&source_path, &target_path).is_ok() {
            let source_files = Args::extract_copy_tagets(&std::path::Path::new(&source_path));
            return Ok(Args {
                source: source_files,
                target: target_path,
                options: None,
            });
        }
        return Err(String::from("Targets are not valid for copy."));
    }
}

#[derive(Debug)]
// TODO: implement various copy options
pub struct CopyOptions {}
