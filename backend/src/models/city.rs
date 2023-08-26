use crate::error::AppError;
use crate::models::page::CityPage;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

/// This struct contains all information regarding a City, including
///     The city name,
///     Growth from the year 2000 to 2013,
///     The Latitude and Longitude,
///     The population
///     The rank, from 1 to 1000, in population,
///     The State in which the city resides in
///
///
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct City {
    /// Name of the City
    pub city: String,

    /// Growth in population from the year 2000 to 2013
    pub growth_from_2000_to_2013: String,

    /// Latitude of the city
    pub latitude: f32,

    /// Longitude of the city
    pub longitude: f32,

    /// Population of the city
    pub population: String,

    /// Rank of the city, between 1 and 1000, 1 being the most populated city, 1000 being 1000th most populated city
    pub rank: String,

    /// Name of the State which the City resides in.
    pub state: String,
}

impl City {
    /// Returns a random City, ranked from 1 to 1000,
    /// The file ```cities.json``` must be present in the path: ``` resources/city_json/cities.json```
    ///
    /// # Returns:
    /// [Self](City)
    ///
    ///
    /// # Examples
    /// ```
    /// let city: City = City::get_random_city();
    ///
    /// println!("City name and populations: {}, {}", city.city, city.population);
    ///
    /// ```
    pub async fn get_random_city() -> Self {
        //let city_file_name = std::env::var("CITY_FILE").expect("Could not find city file.");
        let mut city_file =
            File::open("resources/city_json/cities.json").expect("Could not open city file.");
        let mut city_string = String::new();

        city_file
            .read_to_string(&mut city_string)
            .expect("Could not read file.");

        let city_vec: Vec<City> =
            serde_json::from_str(&city_string).expect("Couldn't parse JSON city file.");

        let mut rng = rand::thread_rng();
        let rand_city_num: u32 = rng.gen_range(1..=1000);

        let chosen_city: City = city_vec
            .iter()
            .find(|&c| c.rank.parse::<u32>().unwrap() == rand_city_num)
            .cloned()
            .unwrap();

        chosen_city
    }

    /// Takes a city rank, as a string, and fetches the corresponding city from the json file
    ///
    /// # Arguments
    /// * `rank` - The city rank, in string form.
    ///
    /// # Returns
    /// Result<[Self](City), [AppError](AppError)>
    ///
    /// # Examples:
    /// ```
    /// let rank: String = String::from("234");
    /// let city: City = City::get_city_with_rank(rank).unwrap();
    /// println!("City name and populations: {}, {}", city.city, city.population);
    /// ```
    pub async fn get_city_with_rank(rank: String) -> Result<Self, AppError> {
        let mut city_file =
            File::open("resources/city_json/cities.json").expect("Could not open city file.");
        let mut city_string = String::new();

        city_file
            .read_to_string(&mut city_string)
            .expect("Could not read file.");

        let city_vec: Vec<City> =
            serde_json::from_str(&city_string).expect("Couldn't parse JSON city file.");

        let chosen_city: City = city_vec.iter().find(|&c| c.rank == rank).cloned().unwrap();

        Ok(chosen_city)
    }

    /// Takes a string, representing the rank of the City desired
    ///
    /// # Arguments:
    /// * rank: String -> Represents the rank of the city, needs to be between "1" and "1000", negative numbers
    ///     or numbers greater than "1000" will Return an AppError
    ///
    /// # Returns:
    /// Result<[CityPage](CityPage), [AppError](AppError)>
    ///
    /// # Examples:
    /// ```rust
    /// let city_rank: String = String::from("954");
    /// let city_page =  City::get_city_in_pagte_from_rank(city_rank);
    ///
    /// match city_page {
    ///     Ok(page) => {
    ///         println!("City: {}", page.city.city);
    ///         page
    ///     }
    ///     Err(err) => {
    ///         println!("Error! {}", err.to_string());
    ///     }
    /// }
    /// ```
    pub async fn get_city_in_page_from_rank(rank: String) -> Result<CityPage, AppError> {
        let city = City::get_city_with_rank(rank).await?;
        let city_page = CityPage { city };
        Ok(city_page)
    }
}
