extern crate github_app;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), github_app::Error> {
    let mut args = env::args();
    let _ = args.next();
    if let (Some(path), Some(installation_id), Some(pr_path)) =
        (args.next(), args.next(), args.next())
    {
        mark_pr_pending(&path, installation_id.parse::<usize>()?, &pr_path)?
    } else {
        println!("Usage: pend_pr path/to/private_key.der $INSTALLATION_ID $PR_PATH");
    }
    Ok(())
}

fn mark_pr_pending(
    path: &str,
    installation_id: usize,
    pull_request_id: &str,
) -> Result<(), github_app::Error> {
    let path: PathBuf = path.into();
    let app = github_app::App::from_private_key_file(&path)?;
    let installation = app.installation(installation_id)?;
    // let repos = installation.repos()?;
    let pr = installation.pull_request(pull_request_id)?;
    // println!("Pull request: {:#?}", pr);
    let context = "GithubApp Test".into();
    if let Some(state) = pr.last_status_for_context(&installation, &context)? {
        if state.state == github_app::pull_request::State::Pending {
            println!("PR already has the desired state!");
            return Ok(())
        }
    }
    // println!("Pull request: {:#?}", pr);
    let status = github_app::pull_request::Status {
        state: github_app::pull_request::State::Pending,
        target_url: "https://example.com".into(),
        description: "This is a test".into(),
        context: context,
    };
    println!("Updating PR to the desired state!");
    pr.set_status(&installation, &status)?;
    Ok(())
}
