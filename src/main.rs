use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};
use std::sync::{Arc, RwLock};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::{Path, Json, State},
    Router,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Movie {
    id: Uuid,
    name: String,
    year: u16,
    was_good: bool,
}

#[derive(Debug, Deserialize)]
struct CreateMoviePayload {
    name: String,
    year: u16,
    was_good: bool
}

#[tokio::main]
async fn main() {
    // Create db with default state.
    let db = Db::default();

    // Set up app with GET `/movie/:id` route and POST `/movie` route with state.
    let app = Router::new().route("/movie/:id", get(get_movie)).route("/movie", post(create_movie)).with_state(db);

    // Set up the addr to bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Run the server.
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

/// Get Movie handler
async fn get_movie(Path(id): Path<Uuid>, State(db): State<Db>) -> Result<impl IntoResponse, StatusCode> {
    // Read the movie if exists. If not, 404 error.
    let movie = db.read().unwrap().get(&id).cloned().ok_or(StatusCode::NOT_FOUND)?;

    // Return the response.
    Ok(Json(movie))
}

/// Create Movie handler
async fn create_movie(State(db): State<Db>, Json(payload): Json<CreateMoviePayload>) -> impl IntoResponse {
    // Create a new movie with a new Uuid.
    let movie = Movie {
        id: Uuid::new_v4(),
        name: payload.name,
        year: payload.year,
        was_good: payload.was_good
    };

    // Write to the db.
    db.write().unwrap().insert(movie.id, movie.clone());

    // Return the response.
    (StatusCode::CREATED, Json(movie))
}

/// Type Alias for Database
type Db = Arc<RwLock<HashMap<Uuid, Movie>>>;
