#![deny(warnings)]

extern crate serde;
#[macro_use] extern crate serde_derive;

extern crate pretty_env_logger;
extern crate warp;

use warp::{Filter};

#[derive(Serialize, Deserialize)]
struct Message {
  id: i64,
  values: Vec<i64>,
  message: String,  
}

const PORT: u16 = 3000;

fn get() -> impl warp::Reply {
  let j = Message {
    id: 8,
    values: vec![1, 2, 3],
    message: "Hello, World!".to_owned(),
  };

  warp::reply::json(&j)
}

fn main() {
  pretty_env_logger::init();

  let get_any = warp::any()
    .map(get)
    .with(warp::reply::with::header("x-rust-is-awesome", "true"))
    .with(warp::log("main::get::any"));
  let post_any = warp::any()
    .and(warp::body::json())
    .map(|msg: Message| {
      warp::reply::json(&msg)
    })
    .with(warp::reply::with::header("x-rust-is-awesome", "true"))
    .with(warp::log("main::post::any"));

  let routes = warp::get(get_any)
    .or(warp::post(post_any));
  let server = warp::serve(routes);

  println!("Warp running on port {}", &PORT);

  server.run(([127, 0, 0, 1], PORT));
}