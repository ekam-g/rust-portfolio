use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};

use rocket::get;
use rocket_dyn_templates::{context, Template};

const META: [&str; 5] = ["Meta - Production Engineering Fellow", "May 2023 - September 2023", "Developed an open-source website template using a tech stack that includes Python, Flask, Jinja, MySQL, Nginx, and unittest.", "Streamlined development processes through automated testing and deployment workflows via CI/CD.", "Established robust system and container monitoring, alerting, and visualization utilizing Prometheus and Grafana."];

const ZIPLINE: [&str; 5] = ["Zipline - Intern", "June 2023 - August 2023", "Contributed as an intern dedicated to enhancing the company's services, leveraging the React framework.", "Worked diligently to improve product reliability, ensuring seamless user experiences.", "Used automation of testing and deployment workows using CI/CD and Git."];

const MLH: [&str; 5] = ["MLH - Software Engineer Prep Fellow","March 2023 - April 2023", "Participated in an immersive 3-week preparation program within the MLH community", "Collaborated with peers to build a portfolio of personal projects, experimenting with technologies such as React, Ruby, and HTML.", "Gained hands-on experience with CI/CD practices and more."];

const CODER_SCHOOL: [&str; 5] = ["Coder School - Teacher", "July 2022 - June 2024", "Serve as a full-time educator, passionately teaching and empowering fellow students.", "Instructed a diverse range of programming languages, including C++, ust, Python, Lua, uby, Java, Golang, JavaScript, React, Elixir, and more.", "Mentored and trained fellow teachers to foster a thriving learning environment."];

const BAD: [&str; 5] = ["ERROR", "ERROR", "ERROR", "ERROR", "ERROR"];

#[get("/<what>")]
pub fn experience(what: i8) -> Template {
    let content = {
        match what {
            0 => META,
            1 => ZIPLINE,
            2 => MLH,
            3 => CODER_SCHOOL,
            _ => BAD,
        }
    };
    Template::render(
        "experience/content",
        context! {job : content[0], time : content[1], list1 : content[2], list2 : content[3], list3 : content[4]},
    )
}
