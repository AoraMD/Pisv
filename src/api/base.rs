use crate::{context::Context, util::extension::ResultExtension, Scope};
use reqwest::{header, Client, Response, Url};
use serde::Deserialize;
use std::{error::Error, path::Path};

use super::authorized_request;

const API_BASE_URL: &str = "https://app-api.pixiv.net";

#[derive(Debug)]
pub(crate) struct IllustrationInfo {
    pub id: u64,
    pub title: String,
    pub artist_id: u64,
    pub artist_name: String,
    pub images: Vec<String>,
}

#[derive(Deserialize)]
struct IllustrationListResp {
    illusts: Vec<IllustrationResp>,
    next_url: Option<String>,
}

#[derive(Deserialize)]
struct IllustrationResp {
    id: u64,
    title: String,
    user: UserResp,
    meta_single_page: MetaSingleResp,
    meta_pages: Vec<MetaPageResp>,
}

#[derive(Deserialize)]
struct UserResp {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct MetaSingleResp {
    original_image_url: Option<String>,
}

#[derive(Deserialize)]
struct MetaPageResp {
    image_urls: ImageUrlsResp,
}

#[derive(Deserialize)]
struct ImageUrlsResp {
    original: String,
}

impl From<IllustrationResp> for IllustrationInfo {
    fn from(value: IllustrationResp) -> Self {
        let mut images: Vec<String> = vec![];
        if let Some(url) = value.meta_single_page.original_image_url {
            images.push(url);
        }
        for meta_page in value.meta_pages {
            images.push(meta_page.image_urls.original);
        }
        return IllustrationInfo {
            id: value.id,
            title: value.title,
            artist_id: value.user.id,
            artist_name: value.user.name,
            images,
        };
    }
}

impl Into<(Vec<IllustrationInfo>, Option<String>)> for IllustrationListResp {
    fn into(self) -> (Vec<IllustrationInfo>, Option<String>) {
        todo!()
    }
}

async fn fetch_like_core(
    client: &Client,
    id: &str,
    scope: Scope,
) -> Result<Response, Box<dyn Error>> {
    let params = [
        ("user_id", id),
        (
            "restrict",
            match scope {
                Scope::Public => "public",
                Scope::Private => "private",
            },
        ),
    ];
    let url = Url::parse_with_params(
        &format!("{}/v1/user/bookmarks/illust", API_BASE_URL),
        &params,
    )?;
    return client.get(url).send().await?.into_ok();
}

pub(crate) async fn fetch_like(
    context: &mut Context,
    id: &str,
    scope: Scope,
) -> Result<(Vec<IllustrationInfo>, Option<String>), Box<dyn Error>> {
    let request = |client: Client| async move { fetch_like_core(&client, id, scope).await };
    let response = {
        let response = authorized_request(context, &request).await?;
        serde_json::from_str::<IllustrationListResp>(&response)?
    };
    let illusts = response
        .illusts
        .into_iter()
        .map(|x| IllustrationInfo::from(x))
        .collect();
    let next = response.next_url;
    return (illusts, next).into_ok();
}

async fn fetch_artist_core(client: &Client, id: u64) -> Result<Response, Box<dyn Error>> {
    let params = [
        ("filter", "for_android"),
        ("type", "illust"),
        ("user_id", &id.to_string()),
    ];
    let url = Url::parse_with_params(&format!("{}/v1/user/illusts", API_BASE_URL), &params)?;
    return client.get(url).send().await?.into_ok();
}

pub(crate) async fn fetch_artist(
    context: &mut Context,
    id: u64,
) -> Result<(Vec<IllustrationInfo>, Option<String>), Box<dyn Error>> {
    let request = |client: Client| async move { fetch_artist_core(&client, id).await };
    let response = {
        let response = authorized_request(context, &request).await?;
        serde_json::from_str::<IllustrationListResp>(&response)?
    };
    let illusts = response
        .illusts
        .into_iter()
        .map(|x| IllustrationInfo::from(x))
        .collect();
    let next = response.next_url;
    return (illusts, next).into_ok();
}

async fn fetch_continue_core(client: &Client, url: &str) -> Result<Response, Box<dyn Error>> {
    return client.get(url).send().await?.into_ok();
}

pub(crate) async fn fetch_continue(
    context: &mut Context,
    url: &str,
) -> Result<(Vec<IllustrationInfo>, Option<String>), Box<dyn Error>> {
    let request = |client: Client| async move { fetch_continue_core(&client, url).await };
    let response = {
        let response = authorized_request(context, &request).await?;
        serde_json::from_str::<IllustrationListResp>(&response)?
    };
    let illusts = response
        .illusts
        .into_iter()
        .map(|x| IllustrationInfo::from(x))
        .collect();
    let next = response.next_url;
    return (illusts, next).into_ok();
}

pub(crate) async fn download_image(file: &Path, url: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(header::REFERER, "https://www.pixiv.net/")
        .send()
        .await?
        .bytes()
        .await?;
    tokio::fs::write(file, response).await?;
    return Ok(());
}
