use crate::configuration::config::Config;
use crate::infrastructure::auth_jwt::Manager;
use crate::modules::auth::constants as auth_constants;
use crate::modules::auth::model::AuthModel;
use crate::modules::auth::schema::LoginRequest;
use crate::modules::users::constants as user_constants;
use crate::modules::users::model::UserModel;
use crate::modules::users::service as user_service;
use bcrypt::BcryptResult;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn login_service(
    pool: &PgPool,
    config: Config,
    body: &LoginRequest,
) -> Result<AuthModel, String> {
    let body_username: String = body.username.to_string();
    let existing_user: Result<UserModel, String> =
        user_service::get_user_by_username_service(pool, body_username.as_str()).await;

    if let Err(error_get_user) = existing_user {
        return match &error_get_user[..] {
            user_constants::USER_NOT_FOUND => {
                // Handle the error
                eprintln!("error get detail user not found {:?}", error_get_user);
                let error_message = user_constants::USER_NOT_FOUND.to_string();
                Err(error_message)
            }
            _ => {
                // Handle the error
                eprintln!("error get detail user {:?}", error_get_user);
                let error_message = user_constants::DETAIL_USER_CANT_BE_FETCHED.to_string();
                Err(error_message)
            }
        };
    }

    let is_equal_password: BcryptResult<bool> = bcrypt::verify(
        body.password.as_str(),
        &existing_user.as_ref().unwrap().password,
    );
    match is_equal_password {
        Ok(password_verified) => {
            if !password_verified {
                let error_message = auth_constants::USERNAME_AND_PASSWORD_FAILED.to_string();
                eprintln!("Error verifying password: {:?}", error_message);
                return Err(error_message);
            }
        }
        Err(err) => {
            let error_message = format!("Error verifying password: {:?}", err);
            eprintln!("{}", error_message);
            return Err(error_message);
        }
    }

    // implement new jwt here and return the access token and refresh token value
    let jwt_manager = match Manager::new(&config.clone()) {
        Ok(manager) => manager,
        Err(err) => {
            eprintln!("Failed to create JWT manager: {:?}", err);
            return Err("Failed to create JWT manager".to_string());
        }
    };
    let access_token = match jwt_manager.new_jwt(&existing_user.as_ref().unwrap().id.to_string()) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to create access token: {:?}", err);
            return Err("Failed to create access token".to_string());
        }
    };
    let refresh_token =
        match jwt_manager.new_refresh_token(&existing_user.as_ref().unwrap().id.to_string()) {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Failed to create refresh token: {:?}", err);
                return Err("Failed to create refresh token".to_string());
            }
        };

    Ok(AuthModel {
        access_token,
        refresh_token,
    })
}

pub async fn refresh_token_service(
    pool: &PgPool,
    config: Config,
    user_id: Uuid,
) -> Result<AuthModel, String> {
    let user_data = user_service::get_user_detail_service(pool, user_id).await;
    if let Err(err) = user_data {
        eprintln!(
            "get user data from authorization header, got error : {}",
            err
        );
        return Err(err);
    }

    // implement new jwt here and return the access token and refresh token value
    let jwt_manager = match Manager::new(&config.clone()) {
        Ok(manager) => manager,
        Err(err) => {
            eprintln!("Failed to create JWT manager: {:?}", err);
            return Err("Failed to create JWT manager".to_string());
        }
    };
    let access_token = match jwt_manager.new_jwt(&user_data.as_ref().unwrap().id.to_string()) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to create access token: {:?}", err);
            return Err("Failed to create access token".to_string());
        }
    };
    let refresh_token =
        match jwt_manager.new_refresh_token(&user_data.as_ref().unwrap().id.to_string()) {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Failed to create refresh token: {:?}", err);
                return Err("Failed to create refresh token".to_string());
            }
        };

    Ok(AuthModel {
        access_token,
        refresh_token,
    })
}
