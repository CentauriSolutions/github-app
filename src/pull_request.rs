use crate::{Account, Repo};
use chrono::prelude::*;

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
    // requested_teams: String, // [],
    pub labels: Vec<String>,
    pub milestone: Option<String>,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
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