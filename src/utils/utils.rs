use crate::configuration::config::Config;
use crate::infrastructure::auth_jwt::Claims;
use actix_web::http::header::HeaderValue;
use actix_web::{HttpRequest, Result};
use jsonwebtoken::{self, decode, Algorithm, DecodingKey, Validation};
use uuid::Uuid;

#[allow(dead_code)]
pub fn get_current_user_uuid_from_jwt(cfg: &Config, req: HttpRequest) -> Result<Uuid> {
    let bearer_header: Option<&HeaderValue> = req.headers().get("Authorization");

    let uuid_zero: Uuid = Uuid::nil();
    if let Some(bearer_header) = bearer_header {
        let bearer_header = bearer_header.to_str().unwrap();
        let header_parts: Vec<&str> = bearer_header.split(' ').collect();

        if header_parts.len() != 2 {
            return Ok(uuid_zero);
        }

        let token_string = header_parts[1];
        if token_string.is_empty() {
            return Ok(uuid_zero);
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
                        Err(_) => Ok(uuid_zero),
                    };
                }
            }
            Err(_) => {
                println!("here 3");
                return Ok(uuid_zero);
            }
        }
    }

    Ok(uuid_zero)
}
