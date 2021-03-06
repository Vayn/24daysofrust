extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use std::io::Read;
use hyper::{Client, HttpResult, HttpError};
use rustc_serialize::{Encodable, json};
use url::form_urlencoded;

fn get_content(url: &str) -> HttpResult<String> {
    let mut client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(HttpError::HttpIoError(e)),
    }
}

type Query<'a> = Vec<(&'a str, &'a str)>;

fn post_query(url: &str, query: Query) -> HttpResult<String> {
    let mut client = Client::new();
    let body = form_urlencoded::serialize(query.into_iter());
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(HttpError::HttpIoError(e)),
    }
}

fn post_json<'a, T>(url: &str, payload: &T) -> HttpResult<String>
    where T: Encodable {
    let mut client = Client::new();
    let body = json::encode(payload).unwrap();
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(HttpError::HttpIoError(e)),
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

fn main() {
    println!("24 days of Rust - hyper (day 5)");
    println!("{:?}", get_content("http://httpbin.org/status/200"));
    let query = vec![("key", "value"), ("foo", "bar")];
    println!("{}", post_query("http://httpbin.org/post", query).unwrap());
    let movie = Movie {
        title: "You Only Live Twice".to_string(),
        bad_guy: "Blofeld".to_string(),
        pub_year: 1967,
    };
    println!("{}", post_json("http://httpbin.org/post", &movie).unwrap());
}
