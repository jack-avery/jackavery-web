#[macro_use]
extern crate rocket;

mod endpoints;
mod error;

use endpoints::hosts::get_hosts;
use endpoints::init;
use endpoints::rasbot::rasbot_notify;

#[launch]
async fn rocket() -> _ {
    init().await;

    rocket::build().mount("/api", routes![get_hosts, rasbot_notify])
}
