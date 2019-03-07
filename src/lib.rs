#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use chrono::prelude::*;
use curl::easy::{Easy, List};

pub use failure::Error;

mod error;
mod json_web_token;

// Github types
mod account;
mod installation;
mod pull_request;
mod repo;

pub use account::Account;
pub use installation::{Installation, Permissions};
pub use pull_request::{PullRequest, PullRequestState};
pub use repo::{Repo, RepoResult};

pub use error::GithubError;
pub use json_web_token::JsonWebToken;

const USER_AGENT: &'static str = "Github App - Rust";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct GithubApp {
    json_web_token: JsonWebToken,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstallationToken {
    pub token: String,
    expires_at: DateTime<Utc>,
}

impl GithubApp {
    pub fn new(private_key: Vec<u8>) -> Result<GithubApp, Error> {
        Ok(GithubApp {
            json_web_token: JsonWebToken::new(private_key)?
        })
    }

    pub fn from_private_key_file(path: &PathBuf) -> Result<GithubApp, Error> {
        Ok(GithubApp {
            json_web_token: JsonWebToken::from_private_key_file(path)?,
        })
    }

    pub fn list_installations(&self) -> Result<Vec<Installation>, Error> {
        let token: String = self.json_web_token.token()?;
        let data = get("https://api.github.com/app/installations", vec![format!("Authorization: Bearer {}", token)])?;
        Ok(serde_json::from_slice(&data)?)
    }

    pub fn list_repos(&self, installation_id: usize) -> Result<Vec<Repo>, Error> {
        let installation_token = self.installation_token(installation_id)?;
        let data = self.get_with_token("https://api.github.com/installation/repositories", &installation_token.token)?;
        println!("Data for repos: {}", String::from_utf8_lossy(&data));
        let mut result: RepoResult = serde_json::from_slice(&data)?;
        let repos = result.repositories.iter_mut().map(|repo| {repo.set_token(installation_token.clone()); repo.clone()});
        Ok(repos.collect())
    }

    fn get_with_token<T1: AsRef<str>, T2: AsRef<str>>(&self, url: T1, token: T2) -> Result<Vec<u8>, Error> {
        get(url, vec![format!("Authorization: token {}", token.as_ref())])
    }

    fn installation_token(&self, installation_id: usize) -> Result<InstallationToken, Error> {
        let token: String = self.json_web_token.token()?;
        let data = post(
            format!("https://api.github.com/app/installations/{}/access_tokens", installation_id),
            vec![format!("Authorization: Bearer {}", token)])?;
        Ok(serde_json::from_slice(&data)?)
    }

}


enum Method {
    Get,
    Post,
}

fn get<T1: AsRef<str>, T2: AsRef<str>>(url: T1, headers: Vec<T2>) -> Result<Vec<u8>, Error> {
    easy_run(url, headers, Method::Get)
}

fn post<T1: AsRef<str>, T2: AsRef<str>>(url: T1, headers: Vec<T2>) -> Result<Vec<u8>, Error> {
    easy_run(url, headers, Method::Post)
}

fn easy_run<T1: AsRef<str>, T2: AsRef<str>>(url: T1, headers: Vec<T2>, method: Method) -> Result<Vec<u8>, Error> {
    let dst = Arc::new(RwLock::new(Vec::with_capacity(8192)));
    let mut easy = Easy::new();
    let url = url.as_ref();
    println!("Getting {}", url);
    easy.url(url)?;

    let mut list = List::new();
    for header in headers {
        list.append(header.as_ref())?;
    };
    list.append(&format!("User-Agent: {}", USER_AGENT))?;
    list.append("Accept: application/vnd.github.machine-man-preview+json")?;
    println!("Headers: {:?}", list);
    easy.http_headers(list)?;
    let inner_dst = dst.clone();
    easy.write_function(move |data| {
        let inner_dst = inner_dst.clone();
        inner_dst.write().unwrap().extend_from_slice(data);
        Ok(data.len())
    })?;
    match method {
        Method::Get => easy.get(true)?,
        Method::Post => easy.post(true)?,
    }
    easy.perform()?;
    let data = (*dst.read().unwrap()).to_vec();
    Ok(data)
}