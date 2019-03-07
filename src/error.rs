use std::io;

use curl;
use jsonwebtoken;

#[derive(Fail, Debug)]
pub enum GithubError {
    #[fail(display = "IO error: {}", error)]
    IoError { error: io::Error },
    #[fail(display = "JWT Error: {}", error)]
    JWTError { error: jsonwebtoken::errors::Error },
    #[fail(display = "Curl Error: {}", error)]
    CurlError { error: curl::Error },
    #[fail(display = "A request was made without a token.")]
    MissingToken,
    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
}
