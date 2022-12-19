use curl::easy::{Easy, List};
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use json::*;

const TOKEN_URL:  &str = "https://open.spotify.com/get_access_token?reason=transport&productType=web_player";
const LYRICS_URL: &str = "https://spclient.wg.spotify.com/color-lyrics/v2/track/";

#[derive(Clone, Debug)]
pub struct Spotify {
    token: String
}

#[derive(Clone, Debug, Copy)]
pub struct SpotifyID<'a> {
    id: &'a str
}

impl Spotify {
    pub fn new(sp_dc: &str) -> Self {
        let mut easy = Easy::new();
        easy.timeout(Duration::from_millis(600)).unwrap();
        easy.ssl_verify_peer(false).unwrap();
        easy.ssl_verify_host(false).unwrap();
        easy.verbose(false).unwrap();
        //return transfer???
        easy.show_header(false).unwrap();
        easy.follow_location(false).unwrap();
        easy.custom_request("GET").unwrap();

        let mut headers = List::new();
        headers.append("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36").unwrap();
        headers.append("App-platform: WebPlayer").unwrap();
        headers.append("content-type: text/html; charset=utf-8").unwrap();
        headers.append(format!("cookie: sp_dc={sp_dc}").as_str()).unwrap();

        easy.http_headers(headers).unwrap();
        easy.url(TOKEN_URL).unwrap();

        let response = Spotify::get_response_string(&mut easy);
        let response = parse(response.as_str()).expect("Invalid SP_DC");
        if response["isAnonymous"].as_bool().unwrap() {
            panic!("Invalid SP_DC");
        }

        Spotify {
            token: String::from(response["accessToken"].as_str().unwrap())
        }
    }

    fn get_response_string(easy: &mut Easy) -> String {
        let buf = Arc::new(Mutex::new(Vec::new()));
        let buf_closure = buf.clone();
        
        easy.write_function(move |data| {
            let mut out = buf_closure.lock().unwrap();
            out.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        easy.perform().expect("Failed to perform request");
        let buf = buf.lock().unwrap().clone();
        let response = String::from_utf8(buf).unwrap();

        response
    }

    pub fn get_lyrics(&self, track_id: &SpotifyID) -> Option<String> {
        let mut easy = Easy::new();
        easy.custom_request("GET").unwrap();

        let mut headers = List::new();
        headers.append("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36").unwrap();
        headers.append("App-platform: WebPlayer").unwrap();
        headers.append("content-type: text/html; charset=utf-8").unwrap();
        headers.append("Accept: application/json; charset=utf-8").unwrap(); /* otherwise gives protobuf */
        headers.append(format!("authorization: Bearer {}", self.token).as_str()).unwrap();
        easy.http_headers(headers).unwrap();

        easy.url(
            format!("{LYRICS_URL}{}?format=json&market=from_token", track_id.id)
                .as_str()
        ).unwrap();
        
        let response = Spotify::get_response_string(&mut easy);
        let response = parse(response.as_str()).expect("Invalid SP_DC");

        Some(String::from(response.dump()))
    }
}

impl SpotifyID<'_> {
    #[allow(dead_code)]
    pub fn from_id(id: &str) -> SpotifyID {
        SpotifyID {id: id}
    }

    #[allow(dead_code)]
    pub fn from_url(url: &str) -> SpotifyID {
        let start = url.find("/track/").expect("Invalid URL") + 7;
        let end = if let Some(pos) = url.find("?") {
            pos
        } else {
            url.len()
        };
        let slice = &url[start..end];

        SpotifyID {id: slice.clone()}
    }
}