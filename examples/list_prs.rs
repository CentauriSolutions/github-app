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
    let app = github_app::App::from_private_key_file(&path, "26261")?;
    let installations = app.installations()?;
    let mut pull_requests = vec![];
    for installation in installations {
        match installation.repos() {
            Ok(repos) => fetch_repos(&installation, repos, &mut pull_requests),
            Err(e) => {
                println!("Had an error fetching repos: {:?}", e);
            }
        }
    }
    println!("Pull Requests:");
    for pr in pull_requests {
        // println!("\t{:?}", pr);
        println!("\t{} - '{}' by {}", pr.url, pr.title, pr.user.login);
    }
    Ok(())
}

fn fetch_repos(
    installation: &github_app::AppInstallation,
    repos: Vec<github_app::Repo>,
    pull_requests: &mut Vec<github_app::PullRequest>,
) {
    for repo in repos {
        match repo.pull_requests(&installation, Some(github_app::PullRequestState::Open)) {
            Ok(mut pulls) => {
                pull_requests.append(&mut pulls);
            }
            Err(e) => {
                println!("Error with Pulls: {:?}", e);
            }
        }
    }
}
