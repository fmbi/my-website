use actix_web::{App, HttpResponse, HttpServer, web, Error};
use actix_web::http::StatusCode;
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

async fn root() -> String {
    "Server is up and running".to_string()
}

async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id: usize = path.into_inner();

    //sqlx::query("SELECT"); -> Query
    
    //sqlx::query_as(""); -> QueryAs

    // sqlx::query!()
    // sqlx::query_as!()

    // .fetch           -> Stream
    // .fetch_all       -> Vec<T>
    // .fetch_option    -> Option<T>
    // .fetch_one       -> T
    // .execute         -> Database::QueryResult -> MySqlQueryResult

    #[derive(sqlx::FromRow)]
    struct User {
        id: i32,
        username: String,
        email: String,
    }

    let user: sqlx::Result<Option<User>> = sqlx::query_as!(
        User,
        "SELECT id, username FROM users WHERE id = ?",
        user_id as u64
        ).fetch_optional(&app_state).await;

    // user.get("username") -> T
    // user.try_get() -> Option<Row>
    ""
}

#[actix_rt::main]
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