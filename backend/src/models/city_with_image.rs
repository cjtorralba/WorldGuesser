use crate::crop_image;
use serde_derive::{Deserialize, Serialize};

use crate::error::AppError;
use crate::models::city::City;
use crate::models::maps::InteractiveMap;
use crate::models::page::PagePackage;

/// Contains a City and an Image String
/// # Parameters:
/// * city - the [City](City) specified
/// * image - An image, specified in a [base64](base64) encoded string, the format of this string is ready
/// to be embedded in html, as it is prefixed with ```data:image/png;base64, image_string```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CityAndImage {
    city: City,
    image: String,
}

impl CityAndImage {
    /// Gets a random City using the [City::get_random_city()](City::get_random_city) then makes a Request to the
    /// [Google Map API](https://developers.google.com/maps/documentation/maps-static/overview) to get a static satellite image.
    /// Request is sent using [Reqwest::get](reqwest::get)
    ///
    /// A Google API KEY must be specified in your ```.env``` file: ```GOOGLE_KEY=your_google_api_key```
    ///
    /// # Returns:
    /// * Result<[Self](CityAndImage), [AppError](AppError)>
    ///
    /// # Examples:
    ///
    ///
    pub async fn get_random_city_and_image() -> Result<Self, AppError> {
        let random_city = City::get_random_city().await;

        let api_key = std::env::var("GOOGLE_KEY").expect("API Key not found!");

        let request_string = format!("https://maps.googleapis.com/maps/api/staticmap?key={}&sensor=true&size=640x400&maptype=satellite&center={},%20{}&zoom=14",
                                     api_key,
                                     random_city.latitude,
                                     random_city.longitude
        );

        let google_response = reqwest::get(request_string).await?;

        let image_bytes = google_response.bytes().await?;

        let image_string = base64::encode(&image_bytes);
        let cropped_string = crop_image(image_string.clone())?;
        let html_string = format!("data:image/png;base64,{}", cropped_string);

        /*  UNCOMMENT IF YOU WANT TO SAVE PICTURES TO "resources/city_images/"

        let image_buffer = image::load_from_memory(&image_bytes)?.to_rgb8();

        let file_name: String = format!("resources/{city}_{lat}_{long}.png",
                                        city = random_city.city,
                                        lat = random_city.latitude,
                                        long = random_city.longitude);
        let mut output_file = File::create(file_name).expect("Failed create image file");

        DynamicImage::ImageRgb8(image_buffer)
            .write_to(&mut output_file, ImageOutputFormat::Png)
            .expect("Could not save image, Error writing bytes.");

         */

        Ok(Self {
            city: random_city,
            image: html_string,
        })
    }

    /// This function returns a City ins PagePackage form, which is what we use with Tera to be able
    /// to view information from within the .html file itself
    ///
    /// # Returns:
    /// * Result<[PagePackage](PagePackage), [AppError](AppError)>
    pub async fn get_city_in_page_form() -> Result<PagePackage, AppError> {
        let city = CityAndImage::get_random_city_and_image().await?;

        let page = PagePackage {
            city_image: city,
            map: InteractiveMap::new(),
        };

        Ok(page)
    }
}
