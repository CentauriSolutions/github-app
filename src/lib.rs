#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use curl::easy::{Easy, List};
use chrono::prelude::*;

pub use failure::Error;

mod error;
mod json_web_token;

pub use error::GithubError;
pub use json_web_token::JsonWebToken;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct GithubApp {
    user_agent: String,
    json_web_token: JsonWebToken,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    login: String,
    id: usize,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Permissions {
    pull_requests: String, // Enum
    metadata: String, // Enum
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Installation {
    id: usize,
    account: Account,
    repository_selection: String, // or Enum?
    access_tokens_url: String,
    repositories_url: String,
    html_url: String,
    app_id: usize,
    target_id: usize,
    target_type: String, // or Enum?
    permissions: Permissions,
    events: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    single_file_name: Option<String>,
}

impl GithubApp {
    pub fn new(private_key: Vec<u8>, user_agent: String) -> Result<GithubApp, Error> {
        Ok(GithubApp {
            json_web_token: JsonWebToken::new(private_key)?,
            user_agent: user_agent
        })
    }

    pub fn from_private_key_file(path: &PathBuf, user_agent: String) -> Result<GithubApp, Error> {
        Ok(GithubApp {
            json_web_token: JsonWebToken::from_private_key_file(path)?,
            user_agent: user_agent
        })
    }

    pub fn list_installations(&self) -> Result<Vec<Installation>, Error> {
        let token: String = self.json_web_token.token()?;
        let data = self.get("app/installations", vec![format!("Authorization: Bearer {}", token)])?;
        Ok(serde_json::from_slice(&data)?)
    }

    fn get<T1: AsRef<str>, T2: AsRef<str>>(&self, url: T1, headers: Vec<T2>) -> Result<Vec<u8>, Error> {
        let dst = Arc::new(RwLock::new(Vec::with_capacity(8192)));
        let mut easy = Easy::new();
        let url = &format!("https://api.github.com/{}", url.as_ref());
        println!("Getting {}", url);
        easy.url(url)?;

        let mut list = List::new();
        for header in headers {
            list.append(header.as_ref())?;
        };
        list.append(&format!("User-Agent: {}", self.user_agent))?;
        list.append("Accept: application/vnd.github.machine-man-preview+json")?;
        println!("Headers: {:?}", list);
        easy.http_headers(list)?;
        let inner_dst = dst.clone();
        easy.write_function(move |data| {
            let inner_dst = inner_dst.clone();
            inner_dst.write().unwrap().extend_from_slice(data);
            Ok(data.len())
        })?;
        easy.perform()?;
        let data = (*dst.read().unwrap()).to_vec();
        Ok(data)
    }
}
