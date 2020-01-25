use crate::course::{list_courses, next_lesson};
use actix_web::{web, App, HttpServer};
use static_files::*;

mod course;
mod static_files;

pub const DEBUG_MODE: bool = true;

fn main() {
    let run = HttpServer::new(|| {
        App::new()
            .service(web::scope("/course").service(next_lesson))
            .service(web::scope("/courses").service(list_courses))
            .service(
                web::scope("/static")
                    // Application JS
                    .service(application_js)
                    // Frameworks
                    .service(bootstrap_css)
                    .service(bootstrap_js)
                    .service(vue_js),
            )
            .service(index)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();

    if let Err(_error) = run {
        println!("Error ")
    }
}
