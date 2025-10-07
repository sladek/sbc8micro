use crate::ui::app::App;
use crate::ui::app::AppState;
use glob::glob;
use std::env;
use std::path::Path;
use std::{fs, fs::ReadDir, io};

pub struct Directory;

impl Directory {
    /// Concatenates an array [&str] to String
    ///
    /// Concatenates an array [&str] to String with items separated by ' '.
    /// It can be used for filename that contains ' ' like "Program Files"
    /// Such a name is nitially stored in an array like this one ["Program", "Files"]
    pub fn concat(params: &[&str]) -> String {
        let mut result = String::new();
        for item in params {
            result.push_str(item);
            result.push(' ');
        }
        result.trim().to_string()
    }
    /// Changes working directory
    pub fn cd(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let filename = Self::concat(&command[1..]);
        if command.len() == 1 {
            let home = env::home_dir();
            match home {
                Some(path) => {
                    let home = path.as_path();
                    let cd = env::set_current_dir(home);
                    match cd {
                        Ok(()) => {}
                        Err(err) => {
                            app.messages.push(err.to_string());
                        }
                    }
                }
                None => {
                    app.messages
                        .push("Cannot change to home directory.".to_string());
                }
            }
            return Ok(AppState::Home);
        }
        let cd = env::set_current_dir(Path::new(&filename));
        match cd {
            Ok(()) => {}
            Err(_) => {
                app.messages.push("Cannot change directory".to_string());
            }
        }
        Ok(AppState::Home)
    }
    /// Shows current working directory
    pub fn pwd(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 1 {
            app.messages
                .push("Invalid number of parameters. Usage: pwd".to_string());
            return Ok(AppState::Home);
        }
        let pwd = env::current_dir();
        match pwd {
            Ok(pwd) => {
                app.messages.push(pwd.display().to_string());
            }
            Err(_) => {
                app.messages
                    .push("Error occured when trying to get current folder.".to_string());
            }
        }
        Ok(AppState::Home)
    }
    /// List a content of directory as ls or dir
    pub fn ls(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let filename = Self::concat(&command[1..]);
        if command.len() != 1 && filename.contains("*") {
            let glob = glob(&filename);
            match glob {
                Ok(glob) => {
                    for entry in glob {
                        match entry {
                            Ok(path) => {
                                app.messages.push(path.display().to_string());
                            }
                            Err(err) => {
                                app.messages.push(err.to_string());
                            }
                        }
                    }
                }
                Err(err) => {
                    app.messages.push(err.to_string());
                }
            }
            return Ok(AppState::Home);
        }
        let paths: io::Result<ReadDir> = 
        if command.len() == 1 {
            fs::read_dir(".")
        } else {
            fs::read_dir(&filename)
        };
        match paths {
            Ok(paths) => {
                for path in paths {
                    match path {
                        Ok(entry) => {
                            let mut filename = entry.path().display().to_string();
                            // Remove leading ./ or .\ if only ls|dir was provided
                            if command.len() == 1
                                && (filename.starts_with(".\\") || filename.starts_with("./"))
                            {
                                filename = filename[2..].to_string();
                            }
                            // Remove leading / or \ even if "ls \" or "ls /"
                            else if filename.starts_with("/") || filename.starts_with("\\") {
                                filename = filename[1..].to_string();
                            }
                            app.messages.push(filename);
                        }
                        Err(err) => {
                            app.messages.push(err.to_string());
                        }
                    }
                }
            }
            Err(err) => {
                app.messages.push(err.to_string());
            }
        }
        Ok(AppState::Home)
    }
}
