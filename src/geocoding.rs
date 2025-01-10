// geocoding.rs
use geocoding::{Forward, GeocodingError, Openstreetmap};

pub fn get_coordinates(place: &String) -> Result<(f64, f64), GeocodingError> {
    // Using Openstreetmaps to return coordinates for a given address
    //
    //
    //
    let osm = Openstreetmap::new();
    let response = match osm.forward(&place) {
        Ok(vector) => vector,
        Err(e) => return Err(e),
    };

    let point = match response.get(0) {
        Some(coordinates) => coordinates,
        None => panic!("No data found for this place!"),
    };
    
    let x = point.y();
    let y = point.x();
    Ok((x,y))
}
