use core::fmt;
use core::ops::Deref;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use chrono::prelude::*;
use curl::easy::{Easy, List};
use failure::Error;

use crate::Installation;
use crate::JsonWebToken;
use crate::PullRequest;
use crate::{Repo, RepoResult};

#[derive(Clone, Debug)]
pub struct App {
    json_web_token: JsonWebToken,
}

impl App {
    pub fn new<T: Into<String>>(private_key: Vec<u8>, app_id: T) -> Result<App, Error> {
        Ok(App {
            json_web_token: JsonWebToken::new(private_key, app_id)?,
        })
    }

    pub fn from_private_key_file<T: Into<String>>(path: &PathBuf, app_id: T) -> Result<App, Error> {
        Ok(App {
            json_web_token: JsonWebToken::from_private_key_file(path, app_id)?,
        })
    }

    pub fn installations(&self) -> Result<Vec<AppInstallation>, Error> {
        let token: String = self.json_web_token.token()?;
        let data = get_with_token("https://api.github.com/app/installations", token)?;
        let installations: Vec<Installation> = serde_json::from_slice(&data)?;
        Ok(installations
            .into_iter()
            .map(|ins| AppInstallation {
                app: self.clone(),
                installation_token: RwLock::new(None),
                installation: ins,
            })
            .collect())
    }

    pub fn installation(&self, installation_id: usize) -> Result<AppInstallation, Error> {
        let token: String = self.json_web_token.token()?;
        let data = get_with_token(
            format!(
                "https://api.github.com/app/installations/{}",
                installation_id
            ),
            token,
        )?;
        let installation: Installation = serde_json::from_slice(&data)?;
        Ok(AppInstallation {
            app: self.clone(),
            installation_token: RwLock::new(None),
            installation: installation,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InstallationToken {
    pub token: String,
    expires_at: DateTime<Utc>,
}

pub struct AppInstallation {
    app: App,
    installation_token: RwLock<Option<InstallationToken>>,
    installation: Installation,
}

impl AppInstallation {
    fn installation_token(&self) -> Result<String, Error> {
        // this annoying jump is to ensure we release the read lock before we try to take the write lock
        let t = {
            match *self.installation_token.read().unwrap() {
                Some(ref t) => Some(t.clone()),
                None => None,
            }
        };
        debug!("Checking if App Installation token is available");
        let token = match t {
            Some(token) => {
                debug!("Token expires at: {}", token.expires_at);
                if token.expires_at >= Utc::now() {
                    debug!("Token expired!");
                    self.refresh_token()?
                } else {
                    token
                }
            }
            None => {
                debug!("No token present, getting one!");
                self.refresh_token()?
            }
        };
        Ok(token.token)
    }

    fn refresh_token(&self) -> Result<InstallationToken, Error> {
        info!("Renewing App Installation token for {}", self.id);
        let token: String = self.app.json_web_token.token()?;
        let data = post(
            &self.access_tokens_url,
            vec![format!("Authorization: Bearer {}", token)],
            None,
        )?;
        let token: InstallationToken = serde_json::from_slice(&data)?;
        trace!("Updated App Installation token for {}", self.id);
        let mut t = self.installation_token.write().unwrap();
        *t = Some(token.clone());
        trace!("Updated stored token");
        Ok(token)
    }

    pub fn repos(&self) -> Result<Vec<Repo>, Error> {
        let data = self.get(&self.repositories_url)?;
        let result: RepoResult = serde_json::from_slice(&data)?;
        Ok(result.repositories)
    }

    /// pull_request_path should be of the form: :owner/:repo/pulls/:number
    pub fn pull_request<'a, T>(&self, pull_request_path: T) -> Result<PullRequest, failure::Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let installation_token = self.installation_token()?;
        let data = get_with_token(
            format!("https://api.github.com/repos/{}", pull_request_path.into()),
            &installation_token,
        )?;
        Ok(serde_json::from_slice(&data)?)
    }

    pub(crate) fn get<T1: AsRef<str>>(&self, url: T1) -> Result<Vec<u8>, Error> {
        let installation_token = self.installation_token()?;
        get(
            url,
            vec![format!("Authorization: token {}", installation_token)],
        )
    }

    pub(crate) fn post<T1: AsRef<str>>(
        &self,
        url: T1,
        body: Option<&[u8]>,
    ) -> Result<Vec<u8>, Error> {
        let installation_token = self.installation_token()?;
        post(
            url,
            vec![format!("Authorization: token {}", installation_token)],
            body,
        )
    }
}

impl Deref for AppInstallation {
    type Target = Installation;

    fn deref(&self) -> &Installation {
        &self.installation
    }
}

impl fmt::Debug for AppInstallation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.installation)
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
}

fn get<T1: AsRef<str>, T2: AsRef<str>>(url: T1, headers: Vec<T2>) -> Result<Vec<u8>, Error> {
    easy_run(url, headers, Method::Get, None)
}

fn get_with_token<T1: AsRef<str>, T2: AsRef<str>>(url: T1, token: T2) -> Result<Vec<u8>, Error> {
    get(
        url,
        vec![format!("Authorization: bearer {}", token.as_ref())],
    )
}

fn post<T1: AsRef<str>, T2: AsRef<str>>(
    url: T1,
    headers: Vec<T2>,
    body: Option<&[u8]>,
) -> Result<Vec<u8>, Error> {
    easy_run(url, headers, Method::Post, body)
}

fn easy_run<T1: AsRef<str>, T2: AsRef<str>>(
    url: T1,
    headers: Vec<T2>,
    method: Method,
    body: Option<&[u8]>,
) -> Result<Vec<u8>, Error> {
    debug!("About to {:?} {}", method, url.as_ref());
    let dst = Arc::new(RwLock::new(Vec::with_capacity(8192)));
    let mut easy = Easy::new();
    let url = url.as_ref();
    easy.url(url)?;

    let mut list = List::new();
    for header in headers {
        list.append(header.as_ref())?;
    }
    list.append(&format!("User-Agent: {}", crate::USER_AGENT))?;
    list.append("Accept: application/vnd.github.machine-man-preview+json")?;
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
    if let Some(data) = body {
        easy.post_fields_copy(data)?;
    }
    easy.perform()?;
    let data = (*dst.read().unwrap()).to_vec();
    debug!("Got {:#?}", String::from_utf8_lossy(&data));
    Ok(data)
}
