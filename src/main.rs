use crate::course::{list_courses, show_course};
use crate::lesson::{list_lessons, show_lesson};
use actix_web::{web, App, HttpServer};
use static_files::*;

mod challenge;
mod course;
mod lesson;
mod static_files;

pub const DEBUG_MODE: bool = true;

fn main() {
    let run = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/course")
                    .service(list_courses)
                    .service(show_course),
            )
            .service(
                web::scope("/lesson")
                    .service(list_lessons)
                    .service(show_lesson),
            )
            .service(
                web::scope("/static")
                    // Application
                    .service(application_css)
                    .service(application_js)
                    // Frameworks
                    .service(bootstrap_css)
                    .service(bootstrap_js)
                    .service(bulma_css)
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
