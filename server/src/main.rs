#[macro_use]
extern crate rocket;

use std::sync::Arc;

use controller::auth_controller;
use controller::main_controller;
use crate::controller::user_controller;

use crate::cors::Cors;

mod config;
mod controller;
mod cors;
mod service;

struct ApiState {
    http_client: Arc<reqwest::Client>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .configure(rocket::Config::figment().merge(("port", config::config().SERVER.PORT)))
        .manage(ApiState {
            http_client: Arc::new(reqwest::Client::new()),
        })
        .mount("/api/auth", auth_controller::routes())
        .mount("/api/user", user_controller::routes())
        .mount("/", main_controller::routes())
}
