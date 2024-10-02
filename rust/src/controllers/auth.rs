use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Form,
};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use serde::Deserialize;
use sqlx::{query, query_as, Pool, Sqlite};

use crate::{
    config::Config,
    models::user::User,
    utilities::{
        app_error::AppError,
        hash::{hash_password, verify_password},
        jwt::create_token,
    },
};

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

//..........................................................
//.LLLL.........OOOOOOO........GGGGGGG...GIIII.NNNN...NNNN..
//.LLLL........OOOOOOOOOO....GGGGGGGGGG..GIIII.NNNNN..NNNN..
//.LLLL.......OOOOOOOOOOOO..GGGGGGGGGGGG.GIIII.NNNNN..NNNN..
//.LLLL.......OOOOO..OOOOO..GGGGG..GGGGG.GIIII.NNNNNN.NNNN..
//.LLLL......LOOOO....OOOOOOGGGG....GGG..GIIII.NNNNNN.NNNN..
//.LLLL......LOOO......OOOOOGGG..........GIIII.NNNNNNNNNNN..
//.LLLL......LOOO......OOOOOGGG..GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL......LOOO......OOOOOGGG..GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL......LOOOO....OOOOOOGGGG.GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL.......OOOOO..OOOOO..GGGGG....GGGGGIIII.NNNN.NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO..GGGGGGGGGGGG.GIIII.NNNN..NNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....GGGGGGGGGG..GIIII.NNNN..NNNNN..
//.LLLLLLLLLL....OOOOOO........GGGGGGG...GIIII.NNNN...NNNN..
//..........................................................

pub async fn login_user(
    State(pool): State<Pool<Sqlite>>,
    State(config): State<Config>,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
        login_form.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to check user: {}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    if let Some(user) = user {
        if !verify_password(&login_form.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid password".to_string(),
            ));
        }

        let token = create_token(&config.jwt_secret, &user.email, &user.role, user.id, 60)?;

        let token_cookie: Cookie = Cookie::build(("token", token))
            .same_site(cookie::SameSite::Lax)
            .http_only(true)
            .path("/")
            .max_age(cookie::time::Duration::minutes(60))
            .into();

        let cookies = CookieJar::new().add(token_cookie);

        match user.role.as_str() {
            "user" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("HX-Location", "/shop/home")
                    .body(axum::body::Body::empty())
                    .unwrap();
                Ok((cookies, response))
            }
            "admin" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("HX-Location", "/admin/dashboard")
                    .body(axum::body::Body::empty())
                    .unwrap();
                Ok((cookies, response))
            }
            _ => {
                tracing::error!("Invalid role");
                Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                ))
            }
        }
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "User not found".to_string(),
        ));
    }
}

//..............................................................................................
//.RRRRRRRRRR...EEEEEEEEEEE....GGGGGGG...GGIII..SSSSSSS....STTTTTTTTTTTEEEEEEEEEE.ERRRRRRRRR....
//.RRRRRRRRRRR..EEEEEEEEEEE..GGGGGGGGGG..GGIII.ISSSSSSSS...STTTTTTTTTTTEEEEEEEEEE.ERRRRRRRRRR...
//.RRRRRRRRRRR..EEEEEEEEEEE.EGGGGGGGGGGG.GGIII.ISSSSSSSSS..STTTTTTTTTTTEEEEEEEEEE.ERRRRRRRRRR...
//.RRRR...RRRRR.EEEE........EGGGG..GGGGG.GGIIIIISSS..SSSS.....TTTT....TEEE........ERRR...RRRRR..
//.RRRR...RRRRR.EEEE.......EEGGG....GGG..GGIIIIISSS...........TTTT....TEEE........ERRR...RRRRR..
//.RRRRRRRRRRR..EEEEEEEEEE.EEGG..........GGIII.ISSSSSS........TTTT....TEEEEEEEEE..ERRRRRRRRRR...
//.RRRRRRRRRRR..EEEEEEEEEE.EEGG..GGGGGGGGGGIII..SSSSSSSSS.....TTTT....TEEEEEEEEE..ERRRRRRRRRR...
//.RRRRRRRR.....EEEEEEEEEE.EEGG..GGGGGGGGGGIII....SSSSSSS.....TTTT....TEEEEEEEEE..ERRRRRRR......
//.RRRR.RRRR....EEEE.......EEGGG.GGGGGGGGGGIII.......SSSSS....TTTT....TEEE........ERRR.RRRR.....
//.RRRR..RRRR...EEEE........EGGGG....GGGGGGIIIIISS....SSSS....TTTT....TEEE........ERRR..RRRR....
//.RRRR..RRRRR..EEEEEEEEEEE.EGGGGGGGGGGG.GGIIIIISSSSSSSSSS....TTTT....TEEEEEEEEEE.ERRR..RRRRR...
//.RRRR...RRRRR.EEEEEEEEEEE..GGGGGGGGGG..GGIII.ISSSSSSSSS.....TTTT....TEEEEEEEEEE.ERRR...RRRRR..
//.RRRR....RRRR.EEEEEEEEEEE....GGGGGGG...GGIII..SSSSSSSS......TTTT....TEEEEEEEEEE.ERRR....RRRR..
//..............................................................................................

pub async fn register_user(
    State(pool): State<Pool<Sqlite>>,
    Form(register_form): Form<RegisterForm>,
) -> impl IntoResponse {
    if register_form.username.is_empty()
        || register_form.email.is_empty()
        || register_form.password.is_empty()
    {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Please fill all fields".to_string(),
        ));
    }

    let result_check_user = query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
        register_form.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to check user: {}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    match result_check_user {
        Some(_) => Err(AppError::new(
            StatusCode::CONFLICT,
            "User already exists".to_string(),
        )),
        None => {
            let password_hash = hash_password(&register_form.password)?;

            query!(
                r#"
                INSERT INTO users (username, email, password)
                VALUES ($1, $2, $3)
                "#,
                register_form.username,
                register_form.email,
                password_hash,
            )
            .execute(&pool)
            .await
            .map_err(|err| {
                tracing::error!("Failed to register user: {}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;

            Ok([(
                "HX-Trigger",
                r#"{"toastmessage":{"type":"success","message":"User create successfully"}}"#,
            )])
        }
    }
}
