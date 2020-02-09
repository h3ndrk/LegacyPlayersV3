#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::json::Json;

#[derive(Debug, Deserialize)]
struct APIToken {
  token: String,
  account_id: u32
}

#[post("/", format = "application/json", data = "<api_token>")]
fn validate_token(api_token: Json<APIToken>) -> Json<bool> {
  Json(api_token.token == "abc" && api_token.account_id == 42)
}

fn main() {
  rocket::ignite()
    .mount("/token_validator/", routes![
      validate_token
    ])
    .launch();
}
