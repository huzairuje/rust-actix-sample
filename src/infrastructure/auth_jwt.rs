use crate::configuration::config::Config;
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken as jwt;
use jwt::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

pub struct Manager {
    signing_key: EncodingKey,
    token_expiry: Duration,
    token_expiry_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Manager {
    pub fn new(cfg: &Config) -> Result<Manager, Box<dyn Error>> {
        if cfg
            .jwt_secret_key
            .as_ref()
            .map_or(true, |key| key.is_empty())
        {
            return Err("empty signing key".into());
        }

        let mut token_expiry: Duration = Duration::from_secs(10 * 3600); // Initialize with default value
        if !cfg.access_token_expiry.is_none() && !cfg.access_token_expiry_unit.is_none() {
            token_expiry = match cfg.access_token_expiry_unit.as_ref().unwrap().as_str() {
                "minutes" => {
                    Duration::from_secs((cfg.access_token_expiry.as_ref().unwrap() * 60) as u64)
                }
                "hours" => {
                    Duration::from_secs((cfg.access_token_expiry.as_ref().unwrap() * 3600) as u64)
                }
                "days" => {
                    Duration::from_secs((cfg.access_token_expiry.as_ref().unwrap() * 86400) as u64)
                }
                _ => Duration::from_secs(10 * 3600),
            };
        }

        let signing_key = EncodingKey::from_secret(cfg.jwt_secret_key.as_ref().unwrap().as_bytes());

        Ok(Manager {
            signing_key,
            token_expiry,
            token_expiry_unit: cfg.access_token_expiry_unit.as_ref().unwrap().clone(),
        })
    }

    pub fn new_jwt(&self, user_id: &str) -> Result<String, Box<dyn Error>> {
        let now = Utc::now().timestamp();
        let expiry = match self.token_expiry_unit.as_str() {
            "minutes" => now + self.token_expiry.as_secs() as i64 / 60,
            "hours" => now + self.token_expiry.as_secs() as i64 / 3600,
            "days" => now + self.token_expiry.as_secs() as i64 / 86400,
            _ => now + 5 * 3600,
        };
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiry as usize,
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.signing_key)?;
        Ok(token)
    }

    pub fn new_refresh_token(&self, user_id: &str) -> Result<String, Box<dyn Error>> {
        let now = Utc::now().timestamp();
        let expiry = now + ChronoDuration::weeks(52).num_seconds();
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiry as usize,
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.signing_key)?;
        Ok(token)
    }
}
