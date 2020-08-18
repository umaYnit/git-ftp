use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    BadTomlData(toml::de::Error),
    DirectoryCreationFailed(std::io::Error),
    NotFoundError(std::io::Error),
    GeneralLoadError(std::io::Error),
    BadConfigDirectoryStr,
    WriteConfigurationFileError(std::io::Error),
    ReadConfigurationFileError(std::io::Error),
    OpenConfigurationFileError(std::io::Error),
    NotFoundServerConfig(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::BadTomlData(e) => write!(f, "Bad TOML data: {}", e),
            CustomError::DirectoryCreationFailed(e) => write!(f, "Failed to create directory: {}", e),
            CustomError::NotFoundError(_) => write!(f, "can not find configuration file, use `init` command or `edit` first."),
            CustomError::GeneralLoadError(_) => write!(f, "Failed to load configuration file."),
            CustomError::BadConfigDirectoryStr => write!(f, "Failed to convert directory name to str."),
            CustomError::WriteConfigurationFileError(_) => write!(f, "Failed to write configuration file."),
            CustomError::ReadConfigurationFileError(_) => write!(f, "Failed to read configuration file."),
            CustomError::OpenConfigurationFileError(_) => write!(f, "Failed to open configuration file."),
            CustomError::NotFoundServerConfig(name) =>write!(f, "Can't find the server name [{}] in config file.",name),
        }
    }
}

impl Error for CustomError {}