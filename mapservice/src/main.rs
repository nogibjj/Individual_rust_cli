use clap::Parser;
use google_maps::prelude::*;

#[derive(Parser)]
//add extended help
#[clap(
    version = "4.0.32",
    author = "Yuxin Song",
    about = "A command line tool for map service.",
    after_help = "Example: cargo run --bin mapservice -- mapservice --function distance --location1 Durham --location2 RDU"
)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(
        about = "Choose a function to use.",
    )]
    Mapservice {
        #[clap(short, long)]
        function: Option<String>, // Determine what language to look at
        #[clap(long, default_value = "Durham")]
        location1: Option<String>,
        #[clap(long, default_value = "RDU")]
        location2: Option<String>,
    },
}
async fn distance(start_location: &str, end_location: &str) {
    let google_maps_client = GoogleMapsClient::new("YOUR API KEY");

    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(start_location)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(end_location)),
            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904)),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;
    let distance = directions.unwrap().routes[0].legs[0].distance.text.clone();
    print!("{}", distance);
}

// create a function to return the distance matrix
async fn locations(start_location: &str, end_location: &str) {
    let google_maps_client = GoogleMapsClient::new("YOUR API KEY");

    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(start_location)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(end_location)),
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
    print!("{}", str);
}

async fn time(start_location: &str, end_location: &str) {
    let google_maps_client = GoogleMapsClient::new("YOUR API KEY");

    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(start_location)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(end_location)),
            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904)),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;
    let duration = directions.unwrap().routes[0].legs[0].duration.text.clone();
    // make response with a body containing the distance
    print!("{}", duration);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Mapservice { function,location1,location2 }) => 
        match function.unwrap().as_str() {
            "distance" => {
                distance(location1.as_ref().unwrap().as_str(),location2.as_ref().unwrap().as_str()).await;
            }
            "time" => {
                time(location1.as_ref().unwrap().as_str(),location2.as_ref().unwrap().as_str()).await;
            }
            "locations" => {
                locations(location1.as_ref().unwrap().as_str(),location2.as_ref().unwrap().as_str()).await;
            }
            &_ => {
                print!("hello");
            }
        },
        _ => {
            print!("hello, world");
        }
    }
    Ok(())
}
