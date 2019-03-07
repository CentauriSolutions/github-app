use crate::get;

use crate::{Account, InstallationToken, PullRequest, PullRequestState};
use crate::error::GithubError;
use chrono::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repo {
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: Account,
    pub permissions: Option<RepoPermission>,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
    pub pushed_at:  DateTime<Utc>,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: usize,
    pub stargazers_count: usize,
    pub watchers_count: usize,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: usize,
    pub mirror_url: Option<String>,
    pub archived: bool,
    pub open_issues_count: usize,
    pub license: Option<String>,
    pub forks: usize,
    pub open_issues: usize,
    pub watchers: usize,
    pub default_branch: String,
    installation_token: Option<InstallationToken>,
}

impl Repo {
    pub fn pull_requests(&self, state: Option<PullRequestState>) -> Result<Vec<PullRequest>, failure::Error> {
        let token = match self.installation_token {
            Some(ref s) => s,
            None => return Err(GithubError::MissingToken)?,
        };
        let mut url = format!("{}?", self.pulls_url.replace("{/number}", ""));
        if let Some(state) = state {
            match state {
                PullRequestState::Open => url = format!("{}&state=open", url),
                PullRequestState::Closed => url = format!("{}&state=closed", url),
            }
        } else {
            url = format!("{}&state=all", url);
        }
        let data = get(url, vec![format!("Authorization: token {}", token.token)])?;
        Ok(serde_json::from_slice(&data)?)
    }

    pub fn set_token(&mut self, token: InstallationToken) {
        self.installation_token = Some(token);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepoPermission {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepoResult {
    pub total_count: usize,
    pub repository_selection: String,
    pub repositories: Vec<Repo>,
}