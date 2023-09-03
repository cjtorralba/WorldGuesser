use crate::error::AppError;
use crate::haversine_distance;
use crate::models::city::City;
use crate::models::city_with_image::CityAndImage;
use crate::models::maps::InteractiveMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use tera::ast::ExprVal::Float;
use tracing::info;
use crate::models::user::UserRankInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
    pub city_image: CityAndImage,
    pub map: InteractiveMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CityPage {
    pub city: City,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DistancePage {
    pub distance: String,
}

impl IntoResponse for DistancePage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl DistancePage {
    pub fn new(city: City, lat: f32, lng: f32) -> Result<Self, AppError> {
        let distance_unrounded: f32 = haversine_distance(city.latitude, city.longitude, lat, lng);
        let distance_rounded_string = format!("{:.3}", distance_unrounded);

        info!("Distance rounded: {}", distance_rounded_string);

        Ok(DistancePage {
            distance: distance_rounded_string,
        })
    }

   pub fn calculate_score(user: UserRankInfo) -> i32 {



       todo!();
   }
}

impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for CityPage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
