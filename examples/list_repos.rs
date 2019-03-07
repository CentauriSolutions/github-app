extern crate github_app;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), github_app::Error> {
    let mut args = env::args();
    let _ = args.next();
    if let (Some(path), Some(installation_id)) = (args.next(), args.next()) {
        list_repos(&path, installation_id.parse::<usize>()?)?;
    } else {
        println!("Usage: list_repos path/to/private_key.der");
    }
    Ok(())
}

fn list_repos(path: &str, installation_id: usize) -> Result<(), github_app::Error> {
    //Vec<String> {
    let path: PathBuf = path.into();
    let app = github_app::GithubApp::from_private_key_file(&path)?;
    println!("{:?}", app.list_repos(installation_id));
    Ok(())
}
