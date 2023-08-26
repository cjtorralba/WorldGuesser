use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub lat: f32,
    pub lng: f32,
    pub city_id: String, // Will need to turn into u32, since it is only deserialized as a string
}
