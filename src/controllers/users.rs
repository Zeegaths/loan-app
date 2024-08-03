
pub async fn register_user(item: web::Form<NewUserForm>, state: web::Data<AppState>) -> impl Responder {
    if item.name.is_empty() || item.email.is_empty() || item.bio.is_empty() || iteem.password.is_empty(){
        return HttpResponse::BadRequest().body("All fields are required");
    }
        //Hash and salt the password

        let hashed_password = match hash(&item.password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),

        };

        let new_user = NewUser {
            name: item.name.clone(),
            email: item.email.clone(),
            bio:item.bio.clone(),
            pasword: hashed_password,
            avatar_url: "".to_string(),
            blocked_reason: "".to_string(),
        };

        let mut connection_guard = stste.db_connection.lock().unwrap();
        let res = add_user(new_user, &mut *connection_guard);

        match res {
            OK(Users) => {
                return handle_login_information("Account created, please login to continue").await;
            }

            Err(err) => {
                error!("Error creating user{:#?", err);
                return handle_register_error("Error Creating Account").await;
            }
        }
    
}