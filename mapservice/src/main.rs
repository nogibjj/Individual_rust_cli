// An actix Microservice that has two routes:
// 1. /mapservice/index
// 2. /mapservice/{location}/{destination}
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use google_maps::prelude::*;
use std::env;

// create a function to return the distance matrix
#[get("/mapservice/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the mapservice!")
}

// create a function to return the distance matrix
#[get("/mapservice/distance/{location}/{destination}")]
async fn distance(path: web::Path<(String, String)>) -> impl Responder {
    let google_maps_client = GoogleMapsClient::new("AIzaSyCo8jSvSmDtiQ1r2zAMSFdYVQs9IoahERw");
    let (location, destination) = path.into_inner();

    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(location)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(destination)),
            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904)),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;
    let distance = directions.unwrap().routes[0].legs[0].distance.text.clone();
    // make response with a body containing the distance
    HttpResponse::Ok().body(distance)
}

// create a function to return the distance matrix
#[get("/mapservice/locations/{location}/{destination}")]
async fn locations(path: web::Path<(String, String)>) -> impl Responder {
    let google_maps_client = GoogleMapsClient::new("AIzaSyCo8jSvSmDtiQ1r2zAMSFdYVQs9IoahERw");
    let (location, destination) = path.into_inner();

    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(location)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(destination)),
            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904)),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;
    let end_location_lat = directions.as_ref().unwrap().routes[0].legs[0]
        .end_location
        .lat
        .clone()
        .to_string();
    let end_location_lng = directions.as_ref().unwrap().routes[0].legs[0]
        .end_location
        .lng
        .clone()
        .to_string();

    let start_location_lat = directions.as_ref().unwrap().routes[0].legs[0]
        .start_location
        .lat
        .clone()
        .to_string();
    let start_location_lng = directions.as_ref().unwrap().routes[0].legs[0]
        .start_location
        .lng
        .clone()
        .to_string();
    // make response with a body containing the distance
    let mut str: String = "The end location Lat is: ".to_owned();
    let str1: &str = &end_location_lat;
    let str3: &str = &"\nThe end location Lng is: ";
    let str2: &str = &end_location_lng;

    str.push_str(str1);
    str.push_str(str3);
    str.push_str(str2);

    let mut str_start_location: String = "\n\nThe start location Lat is: ".to_owned();
    let str1_start: &str = &start_location_lat;
    let str3_start: &str = &"\nThe start location Lng is: ";
    let str2_start: &str = &start_location_lng;

    str_start_location.push_str(str1_start);
    str_start_location.push_str(str3_start);
    str_start_location.push_str(str2_start);
    str.push_str(&str_start_location);
    HttpResponse::Ok().body(str)
}

#[get("/mapservice/time/{location}/{destination}")]
async fn time(path: web::Path<(String, String)>) -> impl Responder {
    let google_maps_client = GoogleMapsClient::new("AIzaSyCo8jSvSmDtiQ1r2zAMSFdYVQs9IoahERw");
    let (location, destination) = path.into_inner();

    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(location)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(destination)),
            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904)),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;
    let duration = directions.unwrap().routes[0].legs[0].duration.text.clone();
    // make response with a body containing the distance
    HttpResponse::Ok().body(duration)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(distance)
            .service(locations)
            .service(time)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
