// use google_maps::prelude::*;

// let google_maps_client = GoogleMapsClient::new("AIzaSyCo8jSvSmDtiQ1r2zAMSFdYVQs9IoahERw");

// let distance_matrix = google_maps_client.distance_matrix(
//     // Origins
//     vec![
//         // Microsoft
//         Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
//         // Cloudflare
//         Waypoint::Address(String::from("101 Townsend St, San Francisco, CA 94107, United States")),
//     ],
//     // Destinations
//     vec![
//         // Google
//         Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
//         // Mozilla
//         Waypoint::LatLng(LatLng::try_from_dec(dec!(37.387_316), dec!(-122.060_008))?),
//     ],
// ).execute().await?;

// // create a function to return the distance matrix
