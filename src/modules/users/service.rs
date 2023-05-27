use crate::infrastructure::password;
use crate::modules::auth::constants as auth_constants;
use crate::modules::users::constants as user_constants;
use crate::modules::users::model::{UserModel, UserSaveModel, UserUpdateModel};
use crate::modules::users::repository;
use crate::modules::users::schema::{CreateUserRequest, UpdateUserRequest, UserResponse};
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn get_user_by_id_service(pool: &PgPool, user_id: Uuid) -> Result<UserModel, String> {
    match repository::get_user_by_id(pool, user_id).await {
        Ok(note) => Ok(note),
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = user_constants::USER_NOT_FOUND;
                    Err(error_message.parse().unwrap())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = user_constants::DETAIL_USER_CANT_BE_FETCHED;
                    Err(error_message.parse().unwrap())
                }
            }
        }
    }
}

pub async fn get_user_detail_service(pool: &PgPool, user_id: Uuid) -> Result<UserResponse, String> {
    let user_data = repository::get_user_by_id(pool, user_id).await;
    match user_data {
        Ok(user) => Ok(UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }),
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail user {:?}", err);
                    let error_message = user_constants::USER_NOT_FOUND;
                    Err(error_message.parse().unwrap())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail user {:?}", err);
                    let error_message = user_constants::DETAIL_USER_CANT_BE_FETCHED;
                    Err(error_message.parse().unwrap())
                }
            }
        }
    }
}

pub async fn get_user_by_username_service(
    pool: &PgPool,
    username: &str,
) -> Result<UserModel, String> {
    match repository::get_user_single_by_username(pool, username.to_string()).await {
        Ok(user) => Ok(user),
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = user_constants::USER_NOT_FOUND;
                    Err(error_message.parse().unwrap())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = user_constants::DETAIL_USER_CANT_BE_FETCHED;
                    Err(error_message.parse().unwrap())
                }
            }
        }
    }
}

pub async fn get_user_by_username_with_user_response(
    pool: &PgPool,
    username: &str,
) -> Result<UserResponse, String> {
    let user_detail = repository::get_user_single_by_username(pool, username.to_string()).await;
    let user_response = match user_detail {
        Ok(user) => UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        },
        Err(err) => {
            return match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = user_constants::USER_NOT_FOUND;
                    Err(error_message.parse().unwrap())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = user_constants::DETAIL_USER_CANT_BE_FETCHED;
                    Err(error_message.parse().unwrap())
                }
            };
        }
    };
    Ok(user_response)
}

pub async fn register_user_service(
    pool: &PgPool,
    body: &CreateUserRequest,
) -> Result<UserResponse, String> {
    let body_username: String = body.username.to_string();
    let existing_user: Result<Vec<UserModel>, Error> =
        repository::get_user_by_username(pool, body_username).await;
    let list_existing_user: Vec<UserModel> = match existing_user {
        Ok(users) => users,
        Err(err) => {
            // Handle the error
            eprintln!("Error getting existing notes: {:?}", err);
            let error_message = user_constants::EXISTING_USER_CANT_BE_FETCHED;
            return Err(error_message.parse().unwrap());
        }
    };
    if list_existing_user.len() > 0 {
        // Title already exists, handle the error
        eprintln!("username {:?} already exists", body.username);
        let error_message = user_constants::USERNAME_ALREADY_EXIST;
        return Err(error_message.parse().unwrap());
    }

    // Hash the password
    let hashed_password = password::hash(&body.password).map_err(|err| {
        eprintln!("Error hashing password: {:?}", err);
        auth_constants::PASSWORD_HASHING_FAILED
    })?;

    let new_user = UserSaveModel {
        username: body.username.to_string(),
        password: hashed_password,
        fullname: Option::from(body.fullname.as_ref().unwrap().to_string()),
        email: Option::from(body.email.as_ref().unwrap().to_string()),
        phone_number: Option::from(body.phone_number.as_ref().unwrap().to_string()),
    };

    let user_save = repository::save_user(pool, &new_user).await;
    let user_response = match user_save {
        Ok(user) => UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        },
        Err(err) => {
            // Handle the error
            eprintln!("Error saving user: {:?}", err);
            let error_message = user_constants::USER_CANT_BE_SAVED;
            return Err(error_message.parse().unwrap());
        }
    };

    Ok(user_response)
}

pub async fn update_user_service(
    pool: &PgPool,
    body: &UpdateUserRequest,
    user_id: Uuid,
) -> Result<UserResponse, String> {
    let user_exist = repository::get_user_by_id(pool, user_id).await;
    let existing_user: UserModel = match user_exist {
        Ok(user) => user.clone(),
        Err(err) => {
            return match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(user_constants::USER_NOT_FOUND.to_string())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(user_constants::DETAIL_USER_CANT_BE_FETCHED.to_string())
                }
            };
        }
    };

    // Hash the password
    let mut hashed_password: String = "".to_string();
    if !body.password.is_none() {
        hashed_password =
            password::hash(&body.password.as_ref().unwrap().to_string()).map_err(|err| {
                eprintln!("Error hashing password: {:?}", err);
                auth_constants::PASSWORD_HASHING_FAILED
            })?;
    }

    let user_update = UserUpdateModel {
        username: Option::from(body.username.as_ref().unwrap().to_string()),
        password: Option::from(hashed_password),
        fullname: Option::from(body.fullname.as_ref().unwrap().to_string()),
        email: Option::from(body.email.as_ref().unwrap().to_string()),
        phone_number: Option::from(body.phone_number.as_ref().unwrap().to_string()),
    };

    let user = existing_user;
    let user_update = repository::update_user(pool, user_id, &user_update, user).await;
    let user_response = match user_update {
        Ok(user) => UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        },
        Err(err) => {
            // Handle the error
            eprintln!("Error saving user: {:?}", err);
            let error_message = user_constants::USER_CANT_BE_SAVED;
            return Err(error_message.parse().unwrap());
        }
    };
    Ok(user_response)
}

pub async fn deactivate_user_service(pool: &PgPool, user_id: Uuid) -> Result<i32, String> {
    let user_exist: Result<UserModel, Error> = repository::get_user_by_id(pool, user_id).await;
    match user_exist {
        Ok(notes) => notes.clone(),
        Err(err) => {
            return match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(user_constants::USER_NOT_FOUND.to_string())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(user_constants::DETAIL_USER_CANT_BE_FETCHED.to_string())
                }
            };
        }
    };

    match repository::delete_user_by_id(pool, user_id).await {
        Ok(user_row) => Ok(user_row),
        Err(err) => {
            // Handle the error
            eprintln!("Error delete note: {:?}", err);
            Err(user_constants::USER_CANT_BE_DELETE.to_string())
        }
    }
}
