use crate::error::AppError;
use crate::models::city::City;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use tracing::info;

#[allow(deprecated)]
#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct InteractiveMap {
    pub request_string: String,
}

impl InteractiveMap {
    /// This function makes use of the Google API embedded map to provide the user with an interactive map
    /// to make clicks on, these clicks will correspond to the location in which the use wishes to guess.
    pub fn new() -> Self {
        let api_key = std::env::var("GOOGLE_KEY").expect("Could not find google api key!");
        let request_string = format!(
            "https://maps.googleapis.com/maps/api/js?key={}&maptype=satellite&callback=initMap",
            api_key
        );
        Self { request_string }
    }
}

/// A Static satellite image that shows two markers, the one the user guesses, and the actual city
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticGuessMap {
    pub image_string: String,
}

impl IntoResponse for StaticGuessMap {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl StaticGuessMap {
    /// Returns a string which is a base64 encoded png picture from google api
    /// This string is ready for html <img> tag, formatted with "data:image/png;base64, IMAGE_STRING"
    pub async fn get_static_map_with_markers(
        rank: String,
        guess_lat: f32,
        guess_lng: f32,
    ) -> Result<Self, AppError> {
        let city = City::get_city_with_rank(rank.clone()).await?;

        let real_lat: f32 = city.latitude;
        let real_lng: f32 = city.longitude;

        let api_key = std::env::var("GOOGLE_KEY").expect("API Key not found!");

        let request_string = format!(
            "https://maps.googleapis.com/maps/api/staticmap?\
        maptype=satellite&\
        visible={guess_lat},{guess_lng}&\
        visible={real_lat},{real_lng}&\
        size=1000x600&\
        markers=color:blue%7Clabel:Guess%7C{guess_lat},{guess_lng}&\
        markers=color:red%7Ccolor:green%7C{real_lat},{real_lng}&\
        path=color:0x0000ff|weight:5|{guess_lat},{guess_lng}|{real_lat},{real_lng}&\
        key={key}",
            guess_lat = guess_lat,
            guess_lng = guess_lng,
            real_lat = real_lat,
            real_lng = real_lng,
            key = api_key
        );

        info!("Request string: {}", request_string);

        let google_response = reqwest::get(request_string).await?;

        let image_bytes = google_response.bytes().await?;
        let image_string = base64::encode(&image_bytes);

        let html_string = format!("data:image/png;base64,{}", image_string);

        Ok(Self {
            image_string: html_string,
        })
    }
}
