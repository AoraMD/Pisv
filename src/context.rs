use crate::{util::extension::ResultExtension, APP_NAME};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{create_dir_all, read_to_string, File},
    io::{Error as IoError, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct Authorization {
    pub access: String,
    pub refresh: String,
    pub id: String,
}

pub(crate) struct Context {
    debug: bool,
    id: String,
}

const AUTH_FILE: &str = "authorization";
const UNKNOWN_USER_ID: &str = "<unknown>";

fn data_path() -> Result<PathBuf, IoError> {
    let path = dirs::data_local_dir()
        .ok_or(IoError::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported operating system",
        ))?
        .join(APP_NAME);
    if !path.exists() {
        create_dir_all(&path)?;
    }
    return path.into_ok();
}

fn auth_file_path() -> Result<PathBuf, IoError> {
    return data_path()?.join(AUTH_FILE).into_ok();   
}

impl Context {
    pub fn new(debug: bool) -> Self {
        return Context {
            debug,
            id: UNKNOWN_USER_ID.to_string(),
        };
    }

    pub fn report_debug(&self, message: &str) {
        if self.debug {
            println!("{}", message);
        }
    }

    pub fn report_info(&self, message: &str) {
        println!("{}", message);
    }

    pub fn report_error(&self, message: &str) {
        eprintln!("{}", message);
    }

    pub fn current_id(&self) -> String {
        return self.id.to_string();
    }

    pub fn update_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn save_auth(&self, auth: Authorization) -> Result<(), Box<dyn Error>> {
        let path = auth_file_path()?;
        self.report_debug(&format!("authorization is saved to {}", path.display()));
        let mut auth_file = File::create(path)?;
        write!(auth_file, "{}", serde_json::to_string(&auth)?)?;
        return Ok(());
    }

    pub fn load_auth(&self) -> Result<Authorization, Box<dyn Error>> {
        let encoded = read_to_string(auth_file_path()?)?;
        return serde_json::from_str::<Authorization>(&encoded)?.into_ok();
    }

    pub fn clean_auth(&self) -> Result<bool, Box<dyn Error>> {
        let path = auth_file_path()?;
        if path.exists() {
            std::fs::remove_file(path)?;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
    
    pub fn export_auth(&self) -> Result<Option<String>, Box<dyn Error>> {
        let path = auth_file_path()?;
        if path.exists() {
            return Some(read_to_string(path)?).into_ok();
        } else {
            return None.into_ok();
        }
    }
}
