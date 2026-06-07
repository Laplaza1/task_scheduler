mod models;
use models::{*};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Executor};
use dotenv::dotenv;
use time::Date;
use std::env;
use crate::models::select_from_table;







#[tokio::main]
async fn main() {
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

}