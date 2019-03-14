#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

pub use failure::Error;

mod error;
mod json_web_token;

mod app;

// Github types
mod account;
mod installation;
pub mod pull_request;
mod repo;

pub use app::{App, AppInstallation};

pub use account::{Account, Team};
pub use installation::{Installation, Permissions};
pub use pull_request::{PullRequest, PullRequestState};
pub use repo::{Repo, RepoResult};

pub use error::GithubError;
pub use json_web_token::JsonWebToken;

const USER_AGENT: &'static str = "Github App - Rust";
