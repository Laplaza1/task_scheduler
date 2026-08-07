
//for DB
mod models;
use axum_governor::GovernorLayer;
use models::{*};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use crate::routes::{AppPool,};
use crate::tests::{port_checker, postgres_init_test};
use axum_limit::{Limit, LimitState, LimitPerSecond};

//for web
mod routes;
use routes::{*};
use axum::{
    http::{HeaderValue, Method,Uri},routing::{delete, get, post, put}, Router
};

use tower_http::cors::{CorsLayer, AllowOrigin};
use axum_limit::Quota;
mod tests;
use log::{*};
mod Auth;
use Auth::*;

#[tokio::main]
async fn main() {


println!("{:?}",dotenv().ok());
   match simple_logging::log_to_file(
        match env::var("LOG_FILE")
            {
                Ok(x)=>{x},
                Err(error)=>{error!("{error}@ finding log file! ");std::process::exit(1)},

            }, LevelFilter::Info)
                {
                    Ok(_)=>{},
                    Err(error)=>{error!("{error}")}
                };
    info!("Application Starting up ");
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    

    
    //DB Example Setup
    
    

    info!("DB Connection Starting up ");
    let database_url = match env::var("DATABASE_URL")
        {
            Ok(x)=>{x},
            Err(error)=>{error!("{error} occured finding DB_URL please verify Postgres connection");std::process::exit(1)}
        };

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        {
            Ok(x)=>{x}
            Err(error)=>{error!("{error} occured Setting pool please verify Postgres connection");std::process::exit(1)}

        };
        

    
    
    //Example of resetting 
    let development_server = match env::var("InDevelopment")
        {
            Ok(x)=>{x},
            Err(error)=>{error!("{error} occured finding InDevelopment please verify env vars");std::process::exit(1)}
        };
    let should_reset = development_server == "True";

    if should_reset 
        {
            reset_users_table(&pool).await;
        }
    
    info!("DB connection established!");
    info!("Starting Tests & Examples");
    
    postgres_init_test(&pool).await;


    info!("Finished Tests & Examples");

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    
    //Web setup
    let origins = vec![
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_static("http://localhost"),
        HeaderValue::from_static("http://127.0.0.1:5500"),
        HeaderValue::from_static("https://laplaza1.github.io"),
    ];

    
    let pool_state = AppPool
                                    {
                                        pool:pool,
                                        limits:LimitState::<Uri>::default(),
                                        api_quota:Quota::per_second(100)
                                    };
    
    

    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE]) // Allow GET and POST
        .allow_origin(AllowOrigin::list(origins))
        .allow_headers([axum::http::header::CONTENT_TYPE,axum::http::header::COOKIE])
        .allow_credentials(true);

    
    let app = Router::new()

    .route("/user", post(user_)).with_state(pool_state.clone())
    .route("/user", get(get_users)).with_state(pool_state.clone())
    .route("/task", post(task_)).with_state(pool_state.clone())
    .route("/task", get(get_task)).with_state(pool_state.clone())
    .route("/task/del", post(delete_task_)).with_state(pool_state.clone())


    .layer(cors)
    .layer(tower::ServiceBuilder::new()
    .layer(GovernorLayer::default()));

     let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await   
                            {
                                Ok(x)=>{x},
                                Err(error)=>{error!("{error} @ listener function please check");std::process::exit(1)}
                            };

    axum::serve(listener, app).await.unwrap();
    



}

