extern crate github_app;

use std::env;
use std::path::PathBuf;

use log::Level;
use simple_logger;

fn main() -> Result<(), github_app::Error> {
    let mut args = env::args();
    let _ = args.next();
    simple_logger::init_with_level(Level::Trace).unwrap();
    if let Some(path) = args.next() {
        list_installations(&path)?;
    } else {
        println!("Usage: list_installations path/to/private_key.der");
    }
    Ok(())
}

fn list_installations(path: &str) -> Result<(), github_app::Error> {
    let path: PathBuf = path.into();
    let app = github_app::App::from_private_key_file(&path, "26261")?;
    println!("{:#?}", app.installations()?);
    Ok(())
}
