use argon2::Config;

use axum::extract::State;
use axum::response::Html;
use axum::Form;
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, Response, StatusCode};
use hyper::Body;

use jsonwebtoken::Header;

use tera::Context;
use tracing::{error, info};

use crate::db::Store;

use crate::error::AppError;

use crate::models::city::City;
use crate::{get_timestamp_after_8_hours, haversine_distance};

use crate::models::city_with_image::CityAndImage;
use crate::models::leaderboard::LeaderBoard;
use crate::models::location::Location;
use crate::models::maps::StaticGuessMap;
use crate::models::page::DistancePage;
use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS};

use crate::template::TEMPLATES;



/// Root routing page. just the "/" route.
/// # Returns:
/// [Result](Result)<[Html](Html)<[String](String)>, [AppError](AppError)>
///
/// # Arguments:
/// Uses dependency injection to get:
/// * [State](State)
/// * [OptionalClaims](OptionalClaims)
pub async fn root(
    State(_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();

    let template_name = if let Some(claims_data) = claims {
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        info!("IS logged in is true");

        let page = CityAndImage::get_city_in_page_form().await?;
        context.insert("page", &page);
        "pages.html"
    } else {
        // not logged in
        context.insert("is_logged_in", &false);
        info!("IS logged in is false");
        "index.html"
    };

    // Render html template with that context
    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            info!("CONTEXT: {:?}", context);
            error!("Template rendering error: {}", err);
            panic!()
        });

    Ok(Html(rendered))
}

/// ======================================
/// CRUD -> Create - Read - Update - Delete
/// ======================================

pub async fn register(
    State(database): State<Store>,
    Form(mut credentials): Form<UserSignup>,
) -> Result<Response<Body>, AppError> {
    // If either of these sections is invalid...
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

    // Check to see if there is already a user in the database with the given email addr
    let existing_user = database.get_user(&credentials.email).await;

    //If it was okay, then someone is trying to sign up with an existing email
    if let Ok(user_option) = existing_user {
        if let Some(_user) = user_option {
            return Err(AppError::UserAlreadyExists);
        }
    }

    // Here we know that user credentials are valid and that another user does not already exist in the database
    // Time to hash the pass

    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    let hashed_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        // Leaving this empty, like we have, means argon will create a unique salt per password
        salt.as_bytes(),
        &hash_config,
    ) {
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed!")));
        }
    };

    credentials.password = hashed_password;

    let new_user = database.create_user(credentials).await;

    match new_user {
        Ok(_user) => {
            let context = Context::new();
            let template_name = "register.html";

            // Render html template with that context
            let rendered = TEMPLATES
                .render(template_name, &context)
                .unwrap_or_else(|err| {
                    info!("CONTEXT: {:?}", context);
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(rendered.into())
                .expect("Failed to build response.");

            Ok(response)
        }
        Err(_e) => {
            let context = Context::new();

            let template_name = "invalid_register.html";

            // Render html template with that context
            let rendered = TEMPLATES
                .render(template_name, &context)
                .unwrap_or_else(|err| {
                    info!("CONTEXT: {:?}", context);
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(rendered.into())
                .expect("Failed to build response.");

            Ok(response)
        }
    }
}


/// TODO: Do something with the optional claims, highlight the specific users rank if possible, as in the claims is Some
pub async fn leaderboard(
    State(database): State<Store>,
    OptionalClaims(claims): OptionalClaims
    ) -> Result<Response<Body>, AppError> {

    let user_rank_list = database.get_top_num_users(100).await?; // TODO: change this to NOT A MAGIC NUMBER
    let leaderboard = LeaderBoard::new(user_rank_list);


    let mut context = Context::new();

    let template_name = {
        context.insert("leaderboard", &leaderboard);
        "leaderboard.html"
    };

    // Render html template with that context
    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(rendered.into())
        .expect("Failed to build response.");

    Ok(response)
}

pub async fn guess_location(
    State(database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
    Form(location): Form<Location>,
) -> Result<Response<Body>, AppError> {

    database.get_top_num_users(4).await?;

    let mut context = Context::new();

    let template_name = if let Some(claims_data) = claims {
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        let city_page = City::get_city_in_page_from_rank(location.city_id.clone()).await?;

        let distance = DistancePage::new(city_page.city.clone(), location.lat, location.lng)?;

        let map = StaticGuessMap::get_static_map_with_markers(
            location.city_id,
            location.lat,
            location.lng,
        )
        .await?;

        // context.insert("distance", &distance); // NEED TO WRITE INTO_RESPONSE FOR distance
        context.insert("static_map", &map);
        context.insert("distance_page", &distance);
        context.insert("city_page", &city_page);

        "guess.html"
    } else {
        // not logged in
        context.insert("is_logged_in", &false);
        "index.html"
    };

    // Render html template with that context
    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(rendered.into())
        .expect("Failed to build response.");

    Ok(response)
}

pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<User>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user(&creds.email).await?;

    return match existing_user {
        Some(user) => {
            info!("User DID exist");
            let is_password_correct =
                match argon2::verify_encoded(&user.password, creds.password.as_bytes()) {
                    Ok(result) => {
                        info!("Password was correct");
                        result
                    },
                    Err(_) => {
                        return Err(AppError::InternalServerError);
                    }
                };

            if !is_password_correct {
                return Err(AppError::InvalidPassword);
            }

            // Here we have authenticated users identity
            // create jwt token to return
            let claims = Claims {
                id: 0,
                email: creds.email.to_owned(),
                exp: get_timestamp_after_8_hours(),
            };

            info!("Generated Claims: {}", claims);

            let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
                .map_err(|_| AppError::MissingCredentials)?;

            info!("Build token string: {}", token);

            // No longer needed since we are using cookies
            // Ok(Json(json!({"access_token" : token, "type" : "Bearer"})))
            let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

            info!("Built cookie");

            let mut response = Response::builder()
                .status(StatusCode::FOUND)
                .body(Body::empty())
                .unwrap();

            response
                .headers_mut()
                .insert(LOCATION, HeaderValue::from_static("/"));

            response.headers_mut().insert(
                SET_COOKIE,
                HeaderValue::from_str(&cookie.to_string()).unwrap(),
            );

            info!("Built response");

            Ok(response)
        }

        None => {
            info!("User did not exist");
            let mut context = Context::new();

            let template_name = {
                context.insert("is_logged_in", &false);
                "index.html"
            };

            // Render html template with that context
            let rendered = TEMPLATES
                .render(template_name, &context)
                .unwrap_or_else(|err| {
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(rendered.into())
                .expect("Failed to build response.");

            Ok(response)
        }
    };
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the protected area! \n Your claim data is: {}",
        claims
    ))
}
