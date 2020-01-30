use crate::course::{list_courses, show_course, Course, CourseDescription};
use crate::lesson::{list_lessons, show_lesson, Lesson, LessonDescription};
use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer};
use static_files::*;
use std::io::BufWriter;
use crate::challenge::{Challenge, next_batch, verify_answer};

mod challenge;
mod course;
mod lesson;
mod static_files;

pub const DEBUG_MODE: bool = true;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pregenerate_data();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(CookieSession::private(&[0; 32]))
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
                web::scope("/challenge")
                    .service(next_batch)
                    .service(verify_answer)
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
    .run()
    .await
}

fn pregenerate_data() {
    let icelandic = Course {
        description: CourseDescription {
            id: "icelandic".to_string(),
            title: "Icelandic language".to_string()
        },
        lessons: vec![
            Lesson {
                description: LessonDescription {
                    id: "basics".to_string(),
                    title: "Language basics".to_string()
                },
                challenges: vec![
                    Challenge {
                        task: "Do something fancy".to_string(),
                        question: "in blue pants".to_string(),
                        accepted: vec![
                            "This is my solution!".to_string()
                        ]
                    }
                ]
            }
        ]
    };

    let yaml = serde_yaml::to_string(&icelandic).unwrap();

    println!("{}", yaml);
}
