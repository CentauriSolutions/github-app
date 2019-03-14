use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use chrono::prelude::*;
use failure::Error;

use jsonwebtoken::{encode, Algorithm, Header};

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    iat: i64,
    exp: i64,
    iss: &'a str,
}

#[derive(Clone, Debug)]
pub struct JsonWebToken {
    expires: Arc<RwLock<DateTime<Utc>>>,
    token: Arc<RwLock<String>>,
    private_key: Vec<u8>,
    application_id: String,
}

impl JsonWebToken {
    pub fn new<T: Into<String>>(private_key: Vec<u8>, application_id: T) -> Result<JsonWebToken, Error> {
        let application_id = application_id.into();
        let (token, expires_time) = JsonWebToken::generate_token(&private_key, &application_id)?;
        let jwt = JsonWebToken {
            expires: Arc::new(RwLock::new(expires_time)),
            token: Arc::new(RwLock::new(token)),
            private_key: private_key,
            application_id: application_id,
        };

        Ok(jwt)
    }

    pub fn is_expired(&self) -> bool {
        let lock = match self.expires.read() {
            Ok(l) => l,
            Err(e) => {
                println!("Error taking lock: {:?}", e);
                return true;
            }
        };
        debug!("Checking if JWT is expired");
        *lock < Utc::now()
    }

    pub fn token(&self) -> Result<String, Error> {
        if self.is_expired() {
            info!("Renewing Application JSON Web Token");
            self.renew_token()?;
            trace!("Successfully renewed JWT");
        }
        Ok((*self.token.read().unwrap()).to_string())
    }

    fn renew_token(&self) -> Result<(), Error> {
        let (token, expires_time) = JsonWebToken::generate_token(&self.private_key, &self.application_id)?;
        *self.token.write().expect("Couldn't lock token for writing") = token;
        // self.expires = RwLock::new(expires_time);
        *self
            .expires
            .write()
            .expect("Couldn't lock expires for writing") = expires_time;
        Ok(())
    }

    fn generate_token(private_key: &Vec<u8>, application_id: &str) -> Result<(String, DateTime<Utc>), Error> {
        let start = Utc::now();
        let since_the_epoch = start.timestamp();
        let exp = since_the_epoch + (10 * 60);
        let my_claims = Claims {
            iat: since_the_epoch,
            exp: exp,
            iss: application_id,
        };
        let expires_time = start + chrono::Duration::seconds(10 * 60);
        debug!("This JWT expires at {}", expires_time);
        // my_claims is a struct that implements Serialize
        // This will create a JWT using RS256 as algorithm
        let token = encode(&Header::new(Algorithm::RS256), &my_claims, &private_key)?;
        Ok((token, expires_time))
    }

    pub fn from_private_key_file<T: Into<String>>(path: &PathBuf, application_id: T) -> Result<JsonWebToken, Error> {
        let mut file = File::open(path)?;
        let mut contents = vec![];
        file.read_to_end(&mut contents)?;
        JsonWebToken::new(contents, application_id)
    }
}
