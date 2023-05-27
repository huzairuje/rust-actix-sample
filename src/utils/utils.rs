use crate::configuration::config::Config;
use crate::infrastructure::auth_jwt::Claims;
use actix_web::http::header::HeaderValue;
use actix_web::{HttpRequest, Result};
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
use jsonwebtoken::{self, decode, Algorithm, DecodingKey, Validation};
use uuid::Uuid;

#[allow(dead_code)]
pub fn get_current_user_uuid_from_jwt(cfg: &Config, req: HttpRequest) -> Result<Uuid, String> {
    let bearer_header: Option<&HeaderValue> = req.headers().get("Authorization");

    let uuid_zero: Uuid = Uuid::nil();
    if let Some(bearer_header) = bearer_header {
        let bearer_header = bearer_header.to_str().unwrap();
        let header_parts: Vec<&str> = bearer_header.split(' ').collect();

        if header_parts.len() != 2 {
            eprintln!("authorization header invalid!");
            return Err("authorization is invalid".to_string());
        }

        let token_string = header_parts[1];
        if token_string.is_empty() {
            eprintln!("the token is empty");
            return Err("authorization header invalid value!".to_string());
        }

        let secret: &[u8] = cfg
            .jwt_secret_key
            .as_ref()
            .map(String::as_bytes)
            .unwrap_or_default();
        let validation: Validation = Validation::new(Algorithm::HS256);
        let token = decode::<Claims>(token_string, &DecodingKey::from_secret(secret), &validation);
        match token {
            Ok(token_data) => {
                let customer_id = token_data.claims.sub.to_string();
                if !customer_id.is_empty() {
                    let customer_uuid = Uuid::parse_str(&customer_id);
                    return match customer_uuid {
                        Ok(uuid) => Ok(uuid),
                        Err(err) => {
                            return Err(err.to_string());
                        }
                    };
                }
            }
            Err(err) => {
                if err.kind() == &ExpiredSignature {
                    eprintln!("the token has been expired {:?}", err);
                    return Err("your token has been expired, please login".to_string());
                }
                return Err(err.to_string());
            }
        }
    } else {
        eprintln!("authorization header invalid!");
        return Err("authorization is invalid".to_string());
    }

    Ok(uuid_zero)
}
