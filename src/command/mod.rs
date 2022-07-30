pub(crate) mod artist;
pub(crate) mod export;
pub(crate) mod import;
pub(crate) mod like;
pub(crate) mod login;
pub(crate) mod logout;

use crate::{
    api::base::{download_image, IllustrationInfo},
    context::Context,
    util::extension::ResultExtension,
    APP_NAME_IN_PATH,
};
use futures::future::join_all;
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    path::{Path, PathBuf},
};

pub(crate) fn default_save_path(sub: &str) -> String {
    let path = match dirs::picture_dir() {
        Some(dir) => dir.join(APP_NAME_IN_PATH).join(sub),
        None => match dirs::data_local_dir() {
            Some(dir) => dir.clone().join(APP_NAME_IN_PATH).join("data").join(sub),
            None => {
                panic!("failed to load default save dir");
            }
        },
    };
    return path.to_string_lossy().to_string();
}

pub(self) async fn create_artist_download_path(
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
    let mut artist_paths = tokio::fs::read_dir(parent).await?;
    let mut entry = artist_paths.next_entry().await?;
    while entry.is_some() {
        let artist_path = entry.unwrap().path();
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
            tokio::fs::rename(artist_path, path.clone()).await?;
            return path.into_ok();
        }
        entry = artist_paths.next_entry().await?;
    }

    // Not exist.
    let path = parent.clone().join(format!("{}-{}", id, name));
    tokio::fs::create_dir_all(path.clone()).await?;
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

async fn create_and_download_image(
    context: &Context,
    parent: &PathBuf,
    url: &str,
    id: u64,
    index: usize,
) -> bool {
    let file = create_image_path(parent, url, id, index);
    if file.exists() {
        return true;
    }
    if let Err(error) = download_image(&file, url).await {
        context.report_error(&format!(
            "failed to download {} to file {}: {}",
            url,
            file.display(),
            error
        ));
    }
    return false;
}

pub(self) async fn fetch_illustration(
    context: &Context,
    parent: &Path,
    illust: &IllustrationInfo,
) -> bool {
    let artist_path =
        match create_artist_download_path(parent, illust.artist_id, &illust.artist_name).await {
            Ok(path) => path,
            Err(error) => {
                context.report_error(&format!(
                    "failed to create artist directory for {}: {}",
                    illust.artist_name, error
                ));
                return false;
            }
        };

    if illust.images.len() == 1 {
        let url = &illust.images[0];
        return create_and_download_image(context, &artist_path, url, illust.id, 0).await;
    } else {
        let artist_path = &artist_path;
        let futures = illust
            .images
            .iter()
            .enumerate()
            .map(|(index, url)| async move {
                create_and_download_image(context, artist_path, url, illust.id, index + 1).await
            });
        return join_all(futures).await.iter().any(|x| *x);
    }
}
