use crate::pages::index;
use pages::icon;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
pub mod pages;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, icon])
        .mount("/static",  FileServer::from(relative!("/templates/static")))
        .attach(Template::custom(|engines| {
            pages::customize(&mut engines.handlebars);
        }))
}
