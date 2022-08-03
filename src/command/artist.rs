use super::fetch_illustration;
use crate::{api::base, context::Context};
use futures::future::join_all;
use std::path::Path;

pub(crate) async fn main(increment: bool, path: String, id: u64, context: &mut Context) {
    let parent = {
        let path = Path::new(&path);
        if !path.exists() {
            if let Err(error) = tokio::fs::create_dir_all(path).await {
                context.report_error(&format!(
                    "failed to create download directory {}: {}",
                    path.display(),
                    error
                ));
                return;
            }
        }
        path
    };

    let result = match base::fetch_artist(context, id).await {
        Ok(result) => result,
        Err(error) => {
            context.report_error(&format!("failed to fetch artist: {}", error));
            return;
        }
    };
    let mut next = result.1;
    let done_increment = {
        let futures = result.0.iter().map(|illust| {
            context.report_info(&format!(
                "fetching {}(id: {}) from {}(id: {})",
                illust.title, illust.id, illust.artist_name, illust.artist_id
            ));
            fetch_illustration(context, parent, illust)
        });
        join_all(futures).await.iter().any(|x| *x)
    };
    if done_increment && increment {
        context.report_info("skip fetch images due to increment mode");
        return;
    }
    while next.is_some() {
        let result = match base::fetch_continue(context, &next.unwrap()).await {
            Ok(result) => result,
            Err(error) => {
                context.report_error(&format!("failed to fetch artist: {}", error));
                return;
            }
        };
        let done_increment = {
            let futures = result.0.iter().map(|illust| {
                context.report_info(&format!(
                    "fetching {}(id: {}) from {}(id: {})",
                    illust.title, illust.id, illust.artist_name, illust.artist_id
                ));
                fetch_illustration(context, parent, illust)
            });
            join_all(futures).await.iter().any(|x| *x)
        };
        if done_increment && increment {
            context.report_info("skip fetch images due to increment mode");
            return;
        }
        next = result.1;
    }
}
