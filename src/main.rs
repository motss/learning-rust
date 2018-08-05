#![deny(warnings)]

extern crate serde;
#[macro_use] extern crate serde_derive;

extern crate pretty_env_logger;
extern crate warp;

use warp::{
  body,
  Filter,
  reply
};

#[derive(Serialize, Deserialize)]
struct Message {
  id: i64,
  values: Vec<i64>,
  message: String,  
}

#[derive(Serialize, Deserialize)]
struct HotelDataHotels {
  id: i64,
  name: String,
  address: String,
}

#[derive(Serialize, Deserialize)]
struct HotelData {
  id: i64,
  hotels: Vec<HotelDataHotels>,
}

const PORT: u16 = 3000;

fn get() -> impl warp::Reply {
  let j = Message {
    id: 8,
    values: [1, 2, 3].to_vec(),
    message: "Hello, World!".to_owned(),
  };

  reply::json(&j)
}

fn main() {
  pretty_env_logger::init();

  let healthcheck = warp::path("healthcheck")
    .map(|| "OK")
    .with(warp::log("main::get::healthcheck"));

  let get_any = warp::any()
    .map(get)
    .with(reply::with::header("x-rust-is-awesome", "true"))
    .with(warp::log("main::get::any"));

  let post_any = warp::any()
    .and(body::json())
    .map(|msg: Message| {
      reply::json(&msg)
    })
    .with(reply::with::header("x-rust-is-awesome", "true"))
    .with(warp::log("main::post::any"));

  let routes = warp::get(
    healthcheck
      .or(get_any)
  )
    .or(warp::post(post_any));

  println!("Warp running on port {}", &PORT);

  warp::serve(routes)
    .run(([127, 0, 0, 1], PORT));
}