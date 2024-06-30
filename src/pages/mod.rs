use rocket::get;

use rocket_dyn_templates::{context, handlebars, Template};

use self::handlebars::{Handlebars, JsonRender};
#[get("/")]
pub fn index() -> Template {
    Template::render("home", context! {Test : "Yo"})
}

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
