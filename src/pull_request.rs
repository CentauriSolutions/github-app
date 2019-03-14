use failure::Error;

use crate::{Account, AppInstallation, Repo, Team};
use chrono::prelude::*;

fn default_context() -> String {
    "default".to_string()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PullRequest {
    pub user: Account,
    pub head: Ref,
    pub base: Ref,
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub number: usize,
    pub state: PullRequestState,
    pub locked: bool,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub merged_at: Option<DateTime<Utc>>,
    pub merge_commit_sha: String,
    pub assignee: Option<Account>,
    pub assignees: Vec<Account>,
    pub requested_reviewers: Vec<Account>,
    pub requested_teams: Vec<Team>,
    pub labels: Vec<String>,
    pub milestone: Option<String>,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub installation_id: Option<usize>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all="lowercase")]
pub enum State {
    Error,
    Failure,
    Pending,
    Success,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Status {
    pub state: State,
    pub target_url: String,
    pub description: String,
    #[serde(default="default_context")]
    pub context: String,
}

impl PullRequest {
    pub fn statuses(&self, installation: &AppInstallation) -> Result<Vec<Status>, Error> {
        Ok(serde_json::from_slice(&installation.get(&self.statuses_url)?)?)
    }

    pub fn last_status_for_context<T: AsRef<str>>(&self, installation: &AppInstallation, context: T) -> Result<Option<Status>, Error> {
        let context = context.as_ref();
        Ok(self.statuses(&installation)?.into_iter().filter(|c| c.context == context).next())
    }

    pub fn set_status(&self, installation: &AppInstallation, status: &Status) -> Result<(), Error> {
        let json = serde_json::to_string(status)?;
        println!("Setting status to {:#?}", json);
        let body = installation.post(&self.statuses_url, Some(json.as_bytes()))?;
        println!("Body: {}", String::from_utf8_lossy(&body));
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ref {
    pub label: String,
    #[serde(rename = "ref")]
    pub pr_ref: String,
    pub sha: String,
    pub user: Account,
    pub repo: Repo,
}
