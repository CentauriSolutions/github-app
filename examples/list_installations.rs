extern crate github_app;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), github_app::Error> {
    let mut args = env::args();
    let _ = args.next();
    if let Some(path) = args.next() {
        list_installations(&path)?;
    } else {
        println!("Usage: list_installations path/to/private_key.der");
    }
    Ok(())
}

fn list_installations(path: &str) -> Result<(), github_app::Error> {
    //Vec<String> {
    let path: PathBuf = path.into();
    let app = github_app::App::from_private_key_file(&path)?;
    println!("{:?}", app.installations());
    Ok(())
}
