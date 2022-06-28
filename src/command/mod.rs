pub(crate) mod artist;
pub(crate) mod import;
pub(crate) mod export;
pub(crate) mod like;
pub(crate) mod login;
pub(crate) mod logout;

use crate::{
    api::base::{download_image, IllustrationInfo},
    context::Context,
    util::extension::ResultExtension,
    APP_NAME_TITLEIZE,
};
use futures::future::join_all;
use std::{
    error::Error,
    fs::{create_dir_all, read_dir, rename},
    io::{Error as IoError, ErrorKind},
    path::{Path, PathBuf},
};

pub(crate) fn default_save_path(sub: &str) -> String {
    let picture_dir = match dirs::picture_dir() {
        Some(dir) => dir,
        None => {
            panic!("failed to load default save dir");
        }
    };
    let path = picture_dir.join(APP_NAME_TITLEIZE).join(sub);
    if !path.exists() {
        create_dir_all(&path).unwrap();
    }
    return path.to_string_lossy().to_string();
}

pub(self) fn create_artist_download_path(
    parent: &Path,
    id: u64,
    name: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let name = name.replace("/", "@");
    
    if !parent.exists() {
        return Err(IoError::new(ErrorKind::AddrNotAvailable, "parent does not exists").into());
    }

    // Exist.
    let path = parent.clone().join(format!("{}-{}", id, name));
    if path.exists() {
        return path.into_ok();
    }

    // Artist was renamed.
    for artist_path in read_dir(parent)? {
        let artist_path = artist_path?.path();
        if artist_path
            .file_name()
            .ok_or(IoError::new(
                ErrorKind::AddrNotAvailable,
                "file name is not available",
            ))?
            .to_string_lossy()
            .to_string()
            .starts_with(&format!("{}-", id))
        {
            let path = parent.clone().join(format!("{}-{}", id, name));
            rename(artist_path, path.clone())?;
            return path.into_ok();
        }
    }

    // Not exist.
    let path = parent.clone().join(format!("{}-{}", id, name));
    create_dir_all(path.clone())?;
    return path.into_ok();
}

pub(self) fn create_image_path(parent: &Path, source_url: &str, id: u64, index: usize) -> PathBuf {
    let extension = match Path::new(&source_url).extension().and_then(|e| e.to_str()) {
        Some(extension) => format!(".{}", extension),
        None => "".to_string(),
    };
    if index == 0 {
        return parent.clone().join(format!("{}{}", id, extension));
    } else {
        return parent
            .clone()
            .join(format!("{}_{}{}", id, index, extension));
    }
}

pub(self) async fn fetch_illustration(
    context: &Context,
    parent: &Path,
    illust: &IllustrationInfo,
) -> bool {
    let artist_path =
        match create_artist_download_path(parent, illust.artist_id, &illust.artist_name) {
            Ok(path) => path,
            Err(error) => {
                context.report_error(&format!(
                    "failed to create artist directory for {}: {}",
                    illust.artist_name, error
                ));
                return false;
            }
        };

    let image_execute = |url: &str, id: u64, index: usize| -> bool {
        let file = create_image_path(&artist_path, url, id, index);
        if file.exists() {
            return true;
        }
        if let Err(error) = download_image(&file, url) {
            context.report_error(&format!(
                "failed to download {} to file {}: {}",
                url,
                file.display(),
                error
            ));
        }
        return false;
    };

    if illust.images.len() == 1 {
        let url = &illust.images[0];
        return image_execute(url, illust.id, 0);
    } else {
        let futures = illust
            .images
            .iter()
            .enumerate()
            .map(|(index, url)| async move { image_execute(url, illust.id, index + 1) });
        return join_all(futures).await.iter().any(|x| *x);
    }
}
