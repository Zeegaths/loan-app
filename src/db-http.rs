use actix_web::{
    get, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    http::{header::{self, ContentType}, Method, StatusCode},
};
use actix_session::{Session, SessionMiddleware};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Define a simple handler function
async fn get_example() -> impl Responder {
    HttpResponse::Ok().body("get")
}

#[get("/get_example_2")]
async fn get_example_2() -> impl Responder {
    HttpResponse::Ok().body("get is working")
}

async fn post_example() -> impl Responder {
    "post!"
}

async fn put_example() -> impl Responder {
    "put!"
}

async fn delete_example() -> impl Responder {
    "delete!"
}

// Simple index handler
#[get("/welcome")]
async fn welcome(req: HttpRequest, session: Session) -> Result<HttpResponse, actix_web::Error> {
    println!("{req:?}");

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {count}");
        counter += 1;
    }
    session.insert("counter", counter)?;

    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body(include_str!("./static/welcome.html"))
    )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct AppState {
    items: Mutex<Vec<Person>>,
}

async fn get_all_persons(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.items)
}

async fn add_person(person: web::Json<Person>, state: web::Data<AppState>) -> impl Responder {
    let mut persons = state.items.lock().unwrap();
    persons.push(person.into_inner());
    HttpResponse::Ok().finish()
}

async fn delete_person(person: web::Json<Person>, state: web::Data<AppState>) -> impl Responder {
    let mut persons = state.items.lock().unwrap();
    if let Some(pos) = persons.iter().position(|x| x.id == person.id) {
        persons.remove(pos);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn put_person(person: web::Json<Person>, state: web::Data<AppState>) -> impl Responder {
    let mut persons = state.items.lock().unwrap();
    if let Some(pos) = persons.iter_mut().find(|x| x.id == person.id) {
        pos.name = person.name.clone();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/get_persons", web::get().to(get_all_persons))
            .route("/add_persons", web::post().to(add_person))
            .route("/delete_persons", web::delete().to(delete_person))
            .route("/put_persons", web::put().to(put_person))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


    // HttpServer::new(|| {
    //     App::new()
    //         .wrap(SessionMiddleware::new(
    //             actix_session::storage::CookieSessionStore::default(),
    //             Key::generate(),
    //         ))
    //         .service(welcome)
    //         .service(get_example_2)
    //         .route("/get_example", web::get().to(get_example))
    //         .route("/post_example", web::post().to(post_example))
    //         .route("/put_example", web::put().to(put_example))
    //         .route("/delete_example", web::delete().to(delete_example))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
}































// mod models;
// use models::*;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;
// mod schema;
// use crate::models::users::{NewUser, Users};

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

// fn main() {
//     use schema::users::dsl::*;
//     let connection = &mut establish_connection();

//     let new_example = NewUser {
//         name: Some("Zee".to_string()), // Wrapped in Some
//         email: Some("Zee@zee.com".to_string()),     // Wrapped in Some
//         sessiontoken: Some(1),                      // Wrapped in Some
//     };

//     let database_record = diesel::insert_into(schema::users::table)
//         .values(new_example)
//         .get_result::<Users>(connection)
//         .expect("Error saving new user");

//     println!("Inserted example: {:?}", database_record);
// }
