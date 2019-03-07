extern crate github_app;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), github_app::Error> {
    let mut args = env::args();
    let _ = args.next();
    if let Some(path) = args.next() {
        list_prs(&path)?;
    } else {
        println!("Usage: list_prs path/to/private_key.der");
    }
    Ok(())
}

fn list_prs(path: &str) -> Result<(), github_app::Error> {
    //Vec<String> {
    let path: PathBuf = path.into();
    let app = github_app::GithubApp::from_private_key_file(&path)?;
    let installations =  app.list_installations()?;
    let mut pull_requests = vec![];
    for installation in  installations {
        match app.list_repos(installation.id) {
            Ok(repos) => {
                fetch_repos(repos, &mut pull_requests)
            },
            Err(e) => {
                println!("Had an error fetching repos: {:?}", e);
            }
        }
    }
    println!("Pull Requests:");
    for pr in pull_requests {
        println!("\t{:?}", pr);
    }
    Ok(())
}

fn fetch_repos(repos: Vec<github_app::Repo>, pull_requests: &mut Vec<github_app::PullRequest>) {
    for repo in repos {
        match repo.pull_requests(Some(github_app::PullRequestState::Open)) {
            Ok(mut pulls) => {
                println!("Have pulls: {:?}", pulls);
                pull_requests.append(&mut pulls);
            }
            Err(e) => {
                println!("Error with Pulls: {:?}", e);
            }
        }
    }
}