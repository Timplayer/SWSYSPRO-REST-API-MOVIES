use actix_web::{web, HttpResponse, Responder, post, get, delete};
use tokio_postgres::Client;
use crate::models::{Movie,MovieInput};
use std::sync::Mutex;

#[get("/movies")]
async fn get_movies(db: web::Data<Mutex<Client>>) -> impl Responder {
    let client = db.lock().unwrap();

    match client.query("SELECT id, name FROM movies", &[]).await {
        Ok(rows) => {
            let movies: Vec<Movie> = rows.iter().map(|row| Movie {
                id: Some(row.get(0)),
                name: row.get(1),
            }).collect();
            HttpResponse::Ok().json(movies)
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/movies")]
async fn post_movies(db: web::Data<Mutex<Client>>, movie: web::Json<MovieInput>,) -> impl Responder {

    let client = match db.lock() {
        Ok(client) => client,
        Err(poisoned) => poisoned.into_inner(),
    };

    let statement = client.prepare("INSERT INTO movies (name) VALUES ($1) RETURNING id, name").await.unwrap();
    match client.query_one(&statement, &[&movie.name]).await {
        Ok(row) => {
            let movie = Movie {
                id: Some(row.get(0)),
                name: row.get(1),
            };
            HttpResponse::Created().json(movie)
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        },
    }
}


#[get("/movies/id/{id}")]
pub async fn get_movie_by_id(
    db: web::Data<Mutex<Client>>,
    path: web::Path<(i32,)>
) -> impl Responder {

    let id = path.into_inner().0;
    let client = db.lock().unwrap();

    match client.query_one("SELECT id, name FROM movies WHERE id = $1", &[&id]).await {
        Ok(row) => {
            let movie = Movie {
                id: Some(row.get(0)),
                name: row.get(1),
            };
            HttpResponse::Ok().json(movie)
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[get("/movies/name/{name}")]
pub async fn get_movie_by_name(
    db: web::Data<Mutex<Client>>,
    path: web::Path<(String,)>
) -> impl Responder {
    let name = path.into_inner().0.to_lowercase();
    let client = db.lock().unwrap();

    let query = "SELECT id, name FROM movies WHERE LOWER(name) = LOWER($1)";

    match client.query(query, &[&name]).await {
        Ok(rows) => {
            let movies: Vec<Movie> = rows.iter().map(|row| Movie {
                id: Some(row.get(0)),
                name: row.get(1),
            }).collect();
            
            if movies.is_empty() {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().json(movies)
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/movies/id/{id}")]
pub async fn delete_movie_by_id(
    db: web::Data<Mutex<Client>>,
    path: web::Path<(i32,)>
) -> impl Responder {
    let movie_id = path.into_inner().0;
    let client = db.lock().unwrap();

    match client.execute("DELETE FROM movies WHERE id = $1", &[&movie_id]).await {
        Ok(count) => {
            if count == 0 {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::NoContent().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_movies);
    config.service(post_movies);
    config.service(get_movie_by_id);
    config.service(get_movie_by_name);
    config.service(delete_movie_by_id);
}