// use actix_web::{
//     get, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
//     http::{header::{self, ContentType}, Method, StatusCode},
// };
// use actix_session::{Session, SessionMiddleware};
// use serde::{Deserialize, Serialize};
// use std::sync::Mutex;
// use std::io;
// mod schema;


// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Person {
//     id: usize,
//     name: String,
// }

// #[derive(Debug)]
// struct AppState {
//     items: Mutex<Vec<Person>>,
// }
// async fn get_all_persons(state:web::Data<AppState>) -> impl Responder {
//     HttpResponse::Ok().json(&state.items)
// }

// async fn add_persons(state: web::Data<AppState>) -> impl Responder {
//     let all_persons = state.items.lock().unwrap();
//     println!("Persons are {}", all_persons.len());
//     HttpResponse::Ok().finish()
// }

// #[actix_web::main]
// async fn main() -> io::Result<()> {
//     let app_state = web::Data::new(AppState{items:Mutex::new(Vec::new())});
//     HttpServer::new(move|| {
//         App::new()
//         .app_data(app_state.clone())
//         .route("/get_persons", web::get().to(get_all_persons))
//         .route("/add_persons", web::post().to(add_persons))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }



use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use std::io;
use actix_web::web::Data;

#[derive(Debug)]
struct AppState {
    users: Mutex<Vec<User>>,
    loans: Mutex<Vec<Loan>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i32,  // SERIAL PRIMARY KEY in SQL corresponds to i32
    user_name: String,
    email: String,
    session_token: i32,  // INTEGER in SQL corresponds to i32
    password: String,  // VARCHAR(255) in SQL corresponds to String
    loan_limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Loan {
    id: i32,
    user_id: i32,
    amount: i32,
    paid_amount: i32,
    interest_rate: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginCredentials {
    user_name: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoanRepayment {
    loan_id: i32,
    payment_amount: i32,
}
// Register a new user
async fn register_user(new_user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    println!("Checking if username exists: {}", new_user.user_name);

    if users.iter().any(|user| user.user_name == new_user.user_name) {
        println!("Username already exists: {}", new_user.user_name);
        return HttpResponse::BadRequest().json("Username already exists");
    }

    let password_hash = hash(&new_user.password, 4).unwrap();
    let new_user = User {
        id: (users.len() as i32) + 1,  // Generate a new ID based on the vector length
        user_name: new_user.user_name.clone(),
        email: new_user.email.clone(),
        session_token: 0,  // Assuming a default session token value
        password: password_hash,
        loan_limit: new_user.loan_limit,
    };

    users.push(new_user); // Directly push the new user without cloning
    println!("User registered: {:?}", users.last()); // Debugging line to confirm the user is added
    HttpResponse::Ok().json(users.last().unwrap())
}

// Login a user
async fn login_user(credentials: web::Json<LoginCredentials>, state: web::Data<AppState>) -> impl Responder {
    let credentials = credentials.into_inner(); // Extract the inner LoginCredentials value
    let users = state.users.lock().unwrap();

    if let Some(user) = users.iter().find(|user| user.user_name == credentials.user_name) {
        if verify(&credentials.password, &user.password).unwrap() {
            HttpResponse::Ok().json(user)
        } else {
            HttpResponse::BadRequest().json("Invalid password")
        }
    } else {
        HttpResponse::BadRequest().json("User not found")
    }
}

async fn get_user_by_id(user_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    match users.iter().find(|&user| user.id == *user_id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn request_loan(new_loan: web::Json<Loan>, state: web::Data<AppState>) -> impl Responder {
    let mut loans = state.loans.lock().unwrap();
    let users = state.users.lock().unwrap();

    if let Some(user) = users.iter().find(|u| u.id == new_loan.user_id) {
        let total_loans: i32 = loans.iter()
            .filter(|loan| loan.user_id == new_loan.user_id)
            .map(|loan| loan.amount)
            .sum();

        if total_loans + new_loan.amount > user.loan_limit {
            return HttpResponse::BadRequest().json("Loan amount exceeds user's limit");
        } else {
            let new_loan = Loan {
                id: (loans.len() as i32) + 1,
                user_id: new_loan.user_id,
                amount: new_loan.amount,
                paid_amount: 0,
                interest_rate: new_loan.interest_rate,
            };
            loans.push(new_loan.clone());
            return HttpResponse::Ok().json(new_loan);
        }
    } else {
        return HttpResponse::BadRequest().json("User not found");
    }
}

async fn repay_loan(loan_repayment: web::Json<LoanRepayment>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    let mut loans = state.loans.lock().unwrap();

    if let Some(loan) = loans.iter_mut().find(|l| l.id == loan_repayment.loan_id) {
        let repaid_amount = loan_repayment.payment_amount;
        loan.paid_amount += repaid_amount;

        if let Some(user) = users.iter_mut().find(|u| u.id == loan.user_id) {
            user.loan_limit += repaid_amount;

            let remaining_amount = loan.amount - loan.paid_amount;
            if remaining_amount == 0 {
                return HttpResponse::Ok().json(format!("Loan fully repaid. New loan limit: {}", user.loan_limit));
            } else {
                return HttpResponse::Ok().json(format!("Loan partially repaid. {} remaining. New loan limit: {}", remaining_amount, user.loan_limit));
            }
        } else {
            return HttpResponse::NotFound().json("User not found");
        }
    } else {
        return HttpResponse::NotFound().json("Loan not found");
    }
}


async fn check_loan_limit(user_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let loans = state.loans.lock().unwrap();
    let total_loan_amount: i32 = loans.iter()
        .filter(|loan| loan.user_id == *user_id)
        .map(|loan| loan.amount)
        .sum();

    if total_loan_amount > 10000 {
        HttpResponse::Ok().body("Loan limit exceeded")
    } else {
        HttpResponse::Ok().body("Under the limit")
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let state = Data::new(AppState {
        users: Mutex::new(Vec::new()),
        loans: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/users/register", web::post().to(register_user))
            .route("/users/login", web::post().to(login_user))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/loans/request", web::post().to(request_loan))
            .route("/loans/repay", web::post().to(repay_loan))
            .route("/loans/check/{id}", web::get().to(check_loan_limit))

            // .route("/login", web::get().to(login_page))
            // .route("/login", web::post().to(login_user))
            // .route("/register", web::get().to(register_page))
            // .route("/register", web::post().to(register_user))
            // .route("/dashboard", web::post().to(dashboard_page))
            // .default_service(web::to(default_handler))



    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
