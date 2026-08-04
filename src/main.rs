
//for DB
mod models;
use axum_governor::GovernorLayer;
use models::{*};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use crate::routes::{AppPool,};
use crate::tests::postgres_init_test;
use axum_limit::{Limit, LimitState, LimitPerSecond};

//for web
mod routes;
use routes::{*};
use axum::{
    http::{HeaderValue, Method,Uri},routing::{delete, get, post, put}, Router
};
use core::panic;
use tower_http::cors::{CorsLayer, AllowOrigin};
use axum_limit::Quota;
mod tests;
use log::{*};


#[tokio::main]
async fn main() {



    simple_logging::log_to_file(env::var("LOG_FILE").expect("Log file must be set in ENV"), LevelFilter::Info).unwrap();
    info!("Application Starting up ");
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    
    //DB Example Setup
    println!("{:?}",dotenv().ok());
    

    info!("DB Connection Starting up ");
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        {
            Ok(x)=>{x}
            Err(e)=>{error!("{e} occured please verify Postgres connection");panic!("{e}")}

        };
        //.expect("Failed to create pool.Check DB config");

    
    
    //Example of resetting 
    reset_users_table(&pool).await;
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
    
    

    //let allowed_origins:[tower_http::cors::AllowOrigin;2] = ["http://localhost".parse().unwrap(),"http://127.0.0.1:5500".parse().unwrap()];
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

     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    



}

