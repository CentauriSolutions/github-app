use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time;

use failure::Error;

use jsonwebtoken::{encode, Algorithm, Header};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: u64,
    exp: u64,
    iss: String,
}

#[derive(Clone, Debug)]
pub struct JsonWebToken {
    expires: Arc<RwLock<time::SystemTime>>,
    token: Arc<RwLock<String>>,
    private_key: Vec<u8>,
}

impl JsonWebToken {
    pub fn new(private_key: Vec<u8>) -> Result<JsonWebToken, Error> {
        let (token, expires_time) = JsonWebToken::generate_token(&private_key)?;
        let jwt = JsonWebToken {
            expires: Arc::new(RwLock::new(expires_time)),
            token: Arc::new(RwLock::new(token)),
            private_key: private_key,
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
        *lock < time::SystemTime::now()
    }

    pub fn token(&self) -> Result<String, Error> {
        if self.is_expired() {
            self.renew_token()?;
        }
        Ok((*self.token.read().unwrap()).to_string())
    }

    fn renew_token(&self) -> Result<(), Error> {
        let (token, expires_time) = JsonWebToken::generate_token(&self.private_key)?;
        *self.token.write().expect("Couldn't lock token for writing") = token;
        // self.expires = RwLock::new(expires_time);
        *self
            .expires
            .write()
            .expect("Couldn't lock expires for writing") = expires_time;
        Ok(())
    }

    fn generate_token(private_key: &Vec<u8>) -> Result<(String, time::SystemTime), Error> {
        let start = time::SystemTime::now();
        let since_the_epoch = start
            .duration_since(time::UNIX_EPOCH)
            .expect("Time went backwards");
        let exp = since_the_epoch.as_secs() + (10 * 60);
        let my_claims = Claims {
            iat: since_the_epoch.as_secs(),
            exp: exp,
            iss: "26261".into(),
        };
        let expires_time = start + time::Duration::new(exp, 0);
        // my_claims is a struct that implements Serialize
        // This will create a JWT using RS256 as algorithm
        let token = encode(&Header::new(Algorithm::RS256), &my_claims, &private_key)?;
        Ok((token, expires_time))
    }

    pub fn from_private_key_file(path: &PathBuf) -> Result<JsonWebToken, Error> {
        let mut file = File::open(path)?;
        let mut contents = vec![];
        file.read_to_end(&mut contents)?;
        JsonWebToken::new(contents)
    }
}
