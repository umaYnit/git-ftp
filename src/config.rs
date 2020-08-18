use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process::Command;

use directories::ProjectDirs;

use crate::error::CustomError;

const STD_EDITOR: &str = "notepad";
const APP_NAME: &str = "git_push";


#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub ip: String,
    pub username: String,
    pub password: String,
    pub port: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Conf {
    edit_path: Option<String>,
    server_config: Option<HashMap<String, ServerConfig>>,
}

impl Conf {
    pub fn get_server(&self, name: &str) -> Option<&ServerConfig> {
        self.server_config.as_ref().and_then(|ref server| server.get(name))
    }
}


pub fn edit_configuration() -> Result<(), CustomError> {
    let config_path = get_config_path()?;

    let editor_cmd = load_config()?.edit_path.unwrap_or(STD_EDITOR.into());

    let mut cmd_iter = editor_cmd.split_whitespace();

    let editor = cmd_iter.next().unwrap();
    let args: Vec<_> = cmd_iter.collect();

    let command = Command::new(editor).args(args).arg(config_path).status();

    match command {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!(
                    "Error: editor {:?} was not found. you can set your $EDITOR by `config edit`",
                    editor
                );
                eprintln!("Full error: {:?}", error);
                std::process::exit(1)
            }
            other_error => panic!("failed to open file: {:?}", other_error),
        },
    }
}

fn init_configuration() -> Result<(), CustomError> {
    let path = get_config_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(CustomError::DirectoryCreationFailed)?;
    }

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .map_err(CustomError::OpenConfigurationFileError)?;
    let init_config = r"
edit_path = 'notepad'

[server_config.example]
ip = 'example'
username = 'example'
password = 'example'
port = 22                   # Can be omitted, default 22

";
    f.write_all(init_config.as_bytes()).map_err(CustomError::WriteConfigurationFileError)?;
    Ok(())
}


fn get_config_path() -> Result<PathBuf, CustomError> {
    let project_dirs = ProjectDirs::from("rs", "", APP_NAME)
        .ok_or(CustomError::BadConfigDirectoryStr)?;
    let config_dir_str = project_dirs
        .config_dir()
        .to_str()
        .ok_or(CustomError::BadConfigDirectoryStr)?;
    Ok([config_dir_str, &format!("{}.toml", APP_NAME)].iter().collect())
}


pub fn load_config() -> Result<Conf, CustomError> {
    let path = get_config_path()?;

    match File::open(&path) {
        Ok(mut cfg) => {
            let mut cfg_string = String::new();
            cfg.read_to_string(&mut cfg_string).map_err(CustomError::ReadConfigurationFileError)?;
            toml::from_str(&cfg_string).map_err(CustomError::BadTomlData)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            init_configuration()?;
            Ok(Conf { edit_path: None, server_config: None })
        }
        Err(e) => Err(CustomError::GeneralLoadError(e)),
    }
}