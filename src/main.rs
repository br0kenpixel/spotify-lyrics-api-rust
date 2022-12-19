mod spotify;
use crate::spotify::*;

fn main() {
    // Cookie
    let sp_dc = "secret";
    // Parse track ID from song link or pass it directly
    let track_id = "0UW7mAWldtWsfVrlFWYYWh";
    //let track_link: String = String::from("https://open.spotify.com/track/0UW7mAWldtWsfVrlFWYYWh?xy=blablablah");

    // Create a client
    let spotify: Spotify = Spotify::new(&sp_dc);
    let track_id: SpotifyID = SpotifyID::from_id(&track_id);
    //let track_id: SpotifyID = SpotifyID::from_url(&track_link);

    let lyrics: String = spotify.get_lyrics(&track_id).unwrap(); /* json */
    println!("{lyrics}");
}