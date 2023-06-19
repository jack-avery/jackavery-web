#[macro_use]
extern crate rocket;

mod endpoints;

use endpoints::init;
use endpoints::hosts::get_hosts;

#[launch]
async fn rocket() -> _ {
    init().await;

    rocket::build().mount("/api", routes![get_hosts])
}
