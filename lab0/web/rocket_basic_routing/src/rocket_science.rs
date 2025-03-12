#[macro_use] extern crate rocket;
// use std::io::{self,Read};
// use std::fs::File;
// use rocket::{response::content::RawHtml};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> Template { // -> Result<String, io::Error>{
//    let mut file = File::open("static/login.html")?;
//    let mut contents = String::new();
//    file.read_to_string(&mut contents)?;
//    // RawHtml(contents) as Result<std::string::String, std::io::Error>
//    Ok(contents)
Template::render("login", context! { field: "value" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![login])
        .attach(Template::fairing())
}

