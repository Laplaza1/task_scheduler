
//for DB
mod models;
use axum::handler::Handler;
use models::{*};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool, Pool, Postgres};
use dotenv::dotenv;
use time::Date;
use std::env;
use crate::models::select_from_table;


//for web
use axum::{
    body::Body, debug_handler, extract::{ws::close_code::STATUS, Path, State}, http::{header::{self, COOKIE, SET_COOKIE}, HeaderMap, HeaderValue, Method, Response, StatusCode}, response::{self, IntoResponse, Json}, routing::{delete, get, post, put}, Router
};
use core::panic;
use std::{any::{type_name, type_name_of_val}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, time::{Duration, SystemTime}};
use axum_extra::extract::{cookie,CookieJar};
use axum_governor::GovernorLayer;
use ::cookie::{Cookie, Expiration, SameSite};
use reqwest;
use tower_http::cors::{CorsLayer, AllowOrigin,Any};
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt, io::Cursor};


#[derive(Clone)]
struct AppPool {
    pool:Pool<Postgres>
}

#[tokio::main]
async fn main() {




    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    
    //DB Example Setup
    println!("{:?}",dotenv().ok());
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    

    //Example of resetting 
    reset_users_table(&pool).await;


    //Example of Creating users
    create_user(&pool, "John Doe", "john@example.com").await.unwrap();

    let user = get_user(&pool, 1).await.unwrap();
    println!("User: {:?}", select_from_table(&pool, models::Tables::User, 1).await.unwrap());

    //Example of updating an Email
    update_user_email(&pool, 1, "john.doe@example.com").await.unwrap();


    //Example of Creating a task
    create_task(&pool, 1, "Make Tacos".to_owned(), "Cook shells and meat and combine with cheese".to_owned(), Date::from_calendar_date(2026, time::Month::June, 30).ok().unwrap()).await.unwrap();

    //Example of deleting a User
    //delete_user(&pool, 1).await.unwrap();


    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    //Web setup
    let origins = vec![
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_static("http://localhost"),
        HeaderValue::from_static("http://127.0.0.1:5500"),
        HeaderValue::from_static("https://laplaza1.github.io"),
    ];

    
    let poolState = AppPool{pool:pool};
    
    //let allowed_origins:[tower_http::cors::AllowOrigin;2] = ["http://localhost".parse().unwrap(),"http://127.0.0.1:5500".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE]) // Allow GET and POST
        .allow_origin(AllowOrigin::list(origins))
        .allow_headers([axum::http::header::CONTENT_TYPE,axum::http::header::COOKIE])
        .allow_credentials(true);

    
    let app = Router::new()

    .route("/user", post(user_)).with_state(poolState.clone())
    .layer(cors);
    // .layer(tower::ServiceBuilder::new()
    //         .layer(GovernorLayer::default()));

     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();




}


//Route Functions

async fn user_(State(poolState): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    

    let name = payload
            .get("name")
            .expect("name doesnt exist")
            .as_str().unwrap();
    
    let email =payload
            .get("email")
            .expect("email doesnt exist")
            .as_str().unwrap();
        


    create_user(
        &poolState.pool,
        name,
        email
        ).await.unwrap(); //<ra@gennew>0)
    return StatusCode::ACCEPTED.into_response()


}