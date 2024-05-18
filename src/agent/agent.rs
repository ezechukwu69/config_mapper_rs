use std::{
    fs,
    io::Error,
    ops::Add,
    process::{Command, ExitStatus, Stdio},
};

use crate::parser::{dto::Config, parser::Parser};

pub struct Agent<'a> {
    pub parser: &'a Parser,
}

impl<'a> Agent<'a> {
    pub fn new(parser: &'a Parser) -> Self {
        Self { parser }
    }

    pub fn run(self: &Self) {
        for config in self.parser.data.item.iter() {
            self.run_for_config(config);
        }
    }

    fn run_for_config(self: &Self, config: &Config) {
        println!("{:#?}", config);
        let path = &config.external.clone();
        let path = path.replace("~", &std::env::var("HOME").unwrap_or("UNKNOWN".into()));
        let target_path = &config.target.clone();
        let target_path =
            target_path.replace("~", &std::env::var("HOME").unwrap_or("UNKNOWN".into()));
        let external_file_metadata = fs::symlink_metadata(&path);
        let target_file_metadata = fs::metadata(&target_path);

        if target_file_metadata.is_err() && external_file_metadata.is_err() && config.repo.is_none()
        {
            eprintln!("[Config Mapper] >>> Invalid config, path and symlink don't exist");
        }

        if target_file_metadata.is_err() && external_file_metadata.is_err() && config.repo.is_some()
        {
            if self
                .clone_git_repo(&target_path, &config.repo.as_ref().unwrap())
                .is_err()
            {
                return;
            }

            if self.create_symlink(&target_path, &path).is_err() {
                return;
            }
        } else if target_file_metadata.is_err()
            && external_file_metadata.is_ok()
            && !external_file_metadata.as_ref().unwrap().is_symlink()
        {
            if config.repo.is_some() {
                if self
                    .clone_git_repo(&target_path, &config.repo.as_ref().unwrap())
                    .is_err()
                {
                    return;
                }
            } else {
                if self.clone_item(&path, &target_path).is_err() {
                    return;
                }
            }

            if self.rename_item(&path).is_err() {
                return;
            }

            if self.delete_item(&path).is_err() {
                return;
            }

            if self.create_symlink(&target_path, &path).is_err() {
                return;
            }
        } else if target_file_metadata.is_ok() && external_file_metadata.is_err() {
            if self.create_symlink(&target_path, &path).is_err() {
                return;
            }
        } else if target_file_metadata.is_ok()
            && external_file_metadata.is_ok()
            && !external_file_metadata.as_ref().unwrap().is_symlink()
        {
            if self.rename_item(&path).is_err() {
                return;
            }

            if self.delete_item(&path).is_err() {
                return;
            }

            if self.create_symlink(&target_path, &path).is_err() {
                return;
            }
        } else if target_file_metadata.is_ok()
            && external_file_metadata.is_ok()
            && external_file_metadata.as_ref().unwrap().is_symlink()
        {
            eprintln!(
                "[Config Mapper] >>> {} >>> skip, symlink already exists",
                config.name
            );
            return;
        }

        println!("<<>>[Config Mapper] >>> {} >>> done", config.name);
    }

    fn rename_item(self: &Self, path: &String) -> std::io::Result<()> {
        fs::rename(path, path.clone().add(".old-config-mapper"))
    }

    fn clone_git_repo(self: &Self, path: &String, repo: &String) -> std::io::Result<ExitStatus> {
        let child_process = Command::new("git")
            .arg("clone")
            .arg(repo)
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn();
        let mut child = match child_process {
            Ok(child_process) => child_process,
            Err(e) => {
                eprintln!("<<>>[Config Mapper] >>> Error cloning git repo - {}", e);
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error cloning git repo".to_string(),
                ));
            }
        };
        let status = child.wait();
        if status.as_ref().is_ok_and(|x| {
            let code = match x.code() {
                Some(c) => c,
                None => 128,
            };
            code != 0
        }) {
            eprintln!(
                "<<>>[Config Mapper] >>> Error cloning git repo - {}",
                status.as_ref().unwrap().code().unwrap()
            );
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Error cloning git repo".to_string(),
            ));
        }
        status
    }

    fn delete_item(self: &Self, path: &String) -> std::io::Result<ExitStatus> {
        let child_process = Command::new("rm").arg("-rf").arg(path).spawn();
        let mut child = match child_process {
            Ok(child_process) => child_process,
            Err(_) => {
                eprintln!("<<>>[Config Mapper] >>> Error deleting item from {}", path);
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error deleting item".to_string(),
                ));
            }
        };
        let status = child.wait();
        status
    }

    fn clone_item(self: &Self, from: &String, to: &String) -> std::io::Result<ExitStatus> {
        let child_process = Command::new("cp").arg("-r").arg(from).arg(to).spawn();
        let mut child = match child_process {
            Ok(child_process) => child_process,
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error cloning item".to_string(),
                ));
            }
        };
        child.wait()
    }

    fn create_symlink(self: &Self, from: &String, to: &String) -> std::io::Result<ExitStatus> {
        let child_process = Command::new("ln").arg("-s").arg(from).arg(to).spawn();
        let mut child = match child_process {
            Ok(child_process) => child_process,
            Err(_) => {
                eprintln!(
                    "<<>>[Config Mapper] >>> Error creating symlink item from {} to {}",
                    from, to
                );
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error creating symlink".to_string(),
                ));
            }
        };
        child.wait()
    }
}
