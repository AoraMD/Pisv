use crate::{util::extension::ResultExtension, APP_NAME_IN_PATH};
use serde::{Deserialize, Serialize};
use std::{error::Error, io::Error as IoError, path::PathBuf};

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

async fn data_path() -> Result<PathBuf, IoError> {
    let path = dirs::data_local_dir()
        .ok_or(IoError::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported operating system",
        ))?
        .join(APP_NAME_IN_PATH);
    if !path.exists() {
        tokio::fs::create_dir_all(&path).await?;
    }
    return path.into_ok();
}

async fn auth_file_path() -> Result<PathBuf, IoError> {
    return data_path().await?.join(AUTH_FILE).into_ok();
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

    pub async fn save_auth(&self, auth: Authorization) -> Result<(), Box<dyn Error>> {
        let path = auth_file_path().await?;
        self.report_debug(&format!("authorization is saved to {}", path.display()));
        let json = serde_json::to_string(&auth)?;
        tokio::fs::write(path, json.as_bytes()).await?;
        return Ok(());
    }

    pub async fn load_auth(&self) -> Result<Authorization, Box<dyn Error>> {
        let encoded = tokio::fs::read_to_string(auth_file_path().await?).await?;
        return serde_json::from_str::<Authorization>(&encoded)?.into_ok();
    }

    pub async fn clean_auth(&self) -> Result<bool, Box<dyn Error>> {
        let path = auth_file_path().await?;
        if path.exists() {
            tokio::fs::remove_file(path).await?;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub async fn export_auth(&self) -> Result<Option<String>, Box<dyn Error>> {
        let path = auth_file_path().await?;
        if path.exists() {
            return Some(tokio::fs::read_to_string(path).await?).into_ok();
        } else {
            return None.into_ok();
        }
    }
}
