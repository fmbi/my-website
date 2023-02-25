use actix_web::{App, HttpResponse, HttpServer, web, Error};
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

async fn home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/index.html"))
    )
}

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    user: User,
    message: String,
}

async fn root() -> String {
    "Server is up and running".to_string()
}

async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let user: Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        user_id as i32
        ).fetch_one(&app_state.pool)
        .await;

    if user.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "No user found with given id.".to_string()
        })
    }

    HttpResponse::Ok().json(UserResponse {
        user: user.unwrap(),
        message: "Got user".to_string(),
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    const DATABASE_URL: &str = "mysql://user:password@127.0.0.1:3306/sqlx";

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(home))
            .route("/root", web::get().to(root))
            .route("/get/{user_id}", web::get().to(get_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}