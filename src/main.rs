#![deny(warnings)]

extern crate serde;
#[macro_use] extern crate serde_derive;

extern crate pretty_env_logger;
extern crate warp;

use warp::{Filter};

static PORT: i64 = 3000;

#[derive(Serialize, Deserialize)]
struct Message {
  id: i64,
  values: Vec<i64>,
  message: String,  
}

fn main() {
  pretty_env_logger::init();

  let get_any = warp::any()
    .map(|| {
      let j = Message {
        id: 8,
        values: vec![1, 2, 3],
        message: "Hello, World!".to_owned(),
      };

      warp::reply::json(&j)
    })
    .with(warp::reply::with::header("x-rust-is-awesome", "true"))
    .with(warp::log("get::any"));

  let routes = warp::get(get_any);
  let server = warp::serve(routes);

  println!("Warp running on port {}", &PORT);

  server.run(([127, 0, 0, 1], 3000));
}