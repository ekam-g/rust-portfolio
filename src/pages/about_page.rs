use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};

use rocket::get;
use rocket_dyn_templates::{context, Template};

static VISTS: AtomicU64 = AtomicU64::new(0);
static UPTIME: AtomicU64 = AtomicU64::new(0);

#[get("/stats/<update>")]
pub fn render(update: bool) -> Template {
    if update {
        VISTS.fetch_add(1, Ordering::Relaxed);
    }
    Template::render(
        "stats",
        context! {vists : VISTS.load(Ordering::Relaxed), uptime : UPTIME.load(Ordering::Relaxed)},
    )
}

pub fn update_uptime() {
    loop {
        UPTIME.fetch_add(1, Ordering::Relaxed);
        thread::sleep(Duration::from_secs(86400));
    }
}
