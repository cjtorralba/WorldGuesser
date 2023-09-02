use crate::db::new_pool;
use crate::error::AppError;
use base64::encode;
use dotenvy::dotenv;
use image::{GenericImageView, ImageOutputFormat};
use std::io::Cursor;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tracing::debug;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod db;
pub mod error;
pub mod handlers;
pub mod layers;
mod models;
mod routes;
mod template;

#[allow(dead_code)]
const EARTH_RADIUS_KM: f32 = 6371.0;
#[allow(dead_code)]
const KM_TO_MILES: f32 = 0.621371;

/// Initializes logging for us, so we can see information about requests
fn init_logging() {
    //From https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=trace,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Gets the host value from the .env file
/// # Returns:
/// * [SocketAddr]
///
/// # .env variables
/// * API_HOST
/// * API_PORT
fn get_host_from_env() -> SocketAddr {
    let host = std::env::var("API_HOST").unwrap();
    let api_host = IpAddr::from_str(&host).unwrap();

    let api_port: u16 = std::env::var("API_PORT").unwrap().parse().unwrap();

    SocketAddr::from((api_host, api_port))
}

/// Runs our backend and initiallizes most of the things we need to run the backend
/// Fetches .env file, initializes logging, gets and binds server address
pub async fn run_backend() {
    // Get environment variables from .env file
    dotenv().ok();
    init_logging();

    let addr = get_host_from_env();

    let app = routes::main_routes::app(new_pool().await).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    debug!("Application initialized");
}

/// This function gets the timestamp of the current time after 8 hours
/// # Returns:
/// * [u64](std::u64)
pub fn get_timestamp_after_8_hours() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backward?");

    let eight_hours_from_now = since_epoch
        + Duration::from_secs(
            60 * /* seconds in a minutes */
            60 * /* Minutes in an hour */
            8, /* Number of hours we want */
        );
    eight_hours_from_now.as_secs()
}

/// This function takes an image string, which in this case will usually be a base64 encoded version of a png
/// It then returns another base64 encoded string, but with 20 pixels cut off from the bottom
///
/// # Arguments:
/// * img_string: [String] - The images you want to crop in [base64](base64) encode format.
///
/// # Returns:
/// * [Result]<[String], [AppError]>
///
/// # Examples:
/// Using a [reqwest::get](reqwest::get) to fetch an image in png format from wherever you would like.
/// ```rust
/// let google_response = reqwest::get("image_request.com").await?;
///
/// let image_bytes = google_response.bytes().await?;
///
/// let image_string = base64::encode(&image_bytes);
/// let cropped_string = crop_image(image_string.clone())?;
///
/// ```
pub fn crop_image(img_str: String) -> Result<String, AppError> {
    let decoded = base64::decode(img_str)?;

    let img = image::load_from_memory(&decoded).unwrap();

    let pixels_to_cut = 20;
    // Get the dimensions of the original image
    let (width, height) = img.dimensions();

    let new_height = height - pixels_to_cut;

    let new_img = img.crop_imm(0, 0, width, new_height);

    let mut encoded = Vec::new();
    let mut cursor = Cursor::new(&mut encoded);
    new_img
        .write_to(&mut cursor, ImageOutputFormat::Png)
        .expect("Failed to encode image");

    let return_string = encode(&encoded);

    Ok(return_string)
}

/// This function calculates the distance between two points, each point has a corresponding latitude and longitude.
/// # Arguments:
/// * lat1: f32 - [Latitude](https://en.wikipedia.org/wiki/Latitude) of fist location
/// * lng1: f32 - [Longitude](https://en.wikipedia.org/wiki/Longitude) of fist location
/// * lat2: f32 - Latitude of second location
/// * lng2: f32 - Longitude of second location
///
/// # Returns:
/// [f32](std::f32)
pub fn haversine_distance(lat1: f32, lng1: f32, lat2: f32, lng2: f32) -> f32 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}

pub type AppResult<T> = Result<T, AppError>;

#[macro_export]
macro_rules! make_db_id {
    ($name:ident) => {
        use derive_more::Display;
        paste::paste! {

            #[derive(
                Clone,
                Copy,
                Debug,
                sqlx::Type,
                Display,
                derive_more::Deref,
                PartialEq,
                Eq,
                Hash,
                Serialize,
                Deserialize,
            )]

            pub struct $name(pub i32);

            impl From<i32> for $name {
                fn from(value: i32) -> Self {
                    $name(value)
                }
            }

            impl From<$name> for i32 {
                fn from(value: $name) -> Self {
                   value.0
                }
            }

            pub trait [<Into $name>] {
                fn into_id(self) -> $name;
            }

            impl [<Into $name>] for i32 {
                fn into_id(self) -> $name {
                    $name::from(self)
                }
            }

            impl [<Into $name>] for $name {
                fn into_id(self) -> $name {
                    self
                }
            }
        }
    };
}
