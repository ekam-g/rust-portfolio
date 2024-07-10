use std::thread;

use crate::pages::index;
use pages::{about_page, content, icon};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
pub mod pages;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    thread::spawn( move || {
        about_page::update_uptime();
    });
    rocket::build()
        .mount("/", routes![index, icon])
        .mount("/about", routes![about_page::render, about_page::langs])
        .mount("/experience", routes![content::experience])
        .mount("/static",  FileServer::from(relative!("/templates/static")))
        .attach(Template::custom(|engines| {
            pages::customize(&mut engines.handlebars);
        }))
}
