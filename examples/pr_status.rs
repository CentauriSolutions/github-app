extern crate github_app;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), github_app::Error> {
    let mut args = env::args();
    let _ = args.next();
    if let (Some(path), Some(installation_id), Some(pr_path)) =
        (args.next(), args.next(), args.next())
    {
        list_status(&path, installation_id.parse::<usize>()?, &pr_path)?
    } else {
        println!("Usage: pend_pr path/to/private_key.der $INSTALLATION_ID $PR_PATH");
    }
    Ok(())
}

fn list_status(
    path: &str,
    installation_id: usize,
    pull_request_id: &str,
) -> Result<(), github_app::Error> {
    let path: PathBuf = path.into();
    let app = github_app::App::from_private_key_file(&path)?;
    let installation = app.installation(installation_id)?;
    // let repos = installation.repos()?;
    let pr = installation.pull_request(pull_request_id)?;
    for status in pr.statuses(&installation)?.iter().filter(|c| c.context == "GithubApp Test") {
        println!("{:#?}", status);
    }
    Ok(())
}
