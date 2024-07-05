pub mod about_page;

use rocket::get;

use self::handlebars::{Handlebars, JsonRender};
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, handlebars, Template};
use std::path::Path;
use rocket::fs::relative;


#[get("/")]
pub fn index() -> Template {
    Template::render("home", context! {})
}
#[get("/favicon.ico")]
pub async fn icon() -> Option<NamedFile> {
    let path = Path::new(relative!("templates/icon.png"));
    NamedFile::open(path).await.ok()
}
//let's handlebars do it's thing
fn wow_helper(
    h: &handlebars::Helper<'_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}
pub fn customize(hbs: &mut Handlebars) {
    hbs.register_helper("wow", Box::new(wow_helper));
}
