use axum::{ extract::{ Path, State }, http::StatusCode, response::IntoResponse, routing::{ get, post, put, delete }, Json, Router };
use serde::{ Deserialize, Serialize };
use sqlx::{ postgres::PgPoolOptions, FromRow, PgPool };
use std::env;

#[derive(Deserialize)]
struct UserPayload {
    name: String,
    email: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect database");
    sqlx::migrate!().run(&pool).await.expect("Failed to migrate database");
    
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server started on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}

// Handlers
async fn root() -> &'static str {
    "Welcome to the User API!"
}

// Get all users
async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

// Get a single user by ID
async fn get_user(Path(id): Path<i32>, State(pool): State<PgPool>) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

// Create a new user
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayload>
) -> Result<impl IntoResponse, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

// Update a user by ID
async fn update_user(
    Path(id): Path<i32>, 
    State(pool): State<PgPool>, 
    Json(payload): Json<UserPayload>
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

// Delete a user by ID
async fn delete_user(Path(id): Path<i32>, State(pool): State<PgPool>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}