use crate::{context::Authorization, util::extension::ResultExtension};
use const_format::formatcp;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use std::error::Error;

const API_BASE_DOMAIN: &str = "oauth.secure.pixiv.net";
const API_BASE_URL: &str = formatcp!("https://{}", API_BASE_DOMAIN);

const CLIENT_ID: &str = "MOBrBDS8blbauoSck0ZfDbtuzpyT";
const CLIENT_SECRET: &str = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj";
const CALLBACK: &str = "https://app-api.pixiv.net/web/v1/users/auth/pixiv/callback";

#[derive(Deserialize)]
struct AuthResp {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResp,
}

#[derive(Deserialize)]
struct UserResp {
    pub id: String,
}

impl From<AuthResp> for Authorization {
    fn from(value: AuthResp) -> Self {
        return Authorization {
            access: value.access_token,
            refresh: value.refresh_token,
            id: value.user.id,
        };
    }
}

fn auth_client() -> Result<Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(header::HOST, HeaderValue::from_static(API_BASE_DOMAIN));
    return super::base_client(headers);
}

pub(crate) async fn login(code: &str, verifier: &str) -> Result<Authorization, Box<dyn Error>> {
    let client = auth_client()?;
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("redirect_uri", CALLBACK),
        ("grant_type", "authorization_code"),
        ("include_policy", "true"),
        ("code", code),
        ("code_verifier", verifier),
    ];
    let response: AuthResp = {
        let response = client
            .post(&format!("{}/auth/token", API_BASE_URL))
            .form(&params)
            .send()
            .await?
            .text()
            .await?;
        serde_json::from_str(&response)?
    };
    return Authorization::from(response).into_ok();
}

pub(crate) async fn refresh(refresh_token: &str) -> Result<Authorization, Box<dyn Error>> {
    let client = auth_client()?;
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("include_policy", "true"),
    ];
    let response: AuthResp = {
        let response = client
            .post(&format!("{}/auth/token", API_BASE_URL))
            .form(&params)
            .send()
            .await?
            .text()
            .await?;
        serde_json::from_str(&response)?
    };
    return Authorization::from(response).into_ok();
}
