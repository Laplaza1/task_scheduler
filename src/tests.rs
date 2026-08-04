use std::panic;

use axum::Error;
use sqlx::{PgPool, Postgres};
use time::Date;
use crate::models::*;
use log::*;
use std::env;




/// Validates that the input X
/// 
/// 
/// '''
/// converts x into Chars which are checked for invalid characters
/// '''
/// 
pub fn verify_normal_chars(x:&String)
    

    {
        if x.chars().any(|x|!x.is_alphanumeric() && !x.is_whitespace())==true
            {
                error!("Entered value isnt safe:{x}. Please use normal characters and retry.")
            }

    }

/// Initalization Testing for postgres
/// 

pub async fn postgres_init_test(pool: &PgPool)
    {
            simple_logging::log_to_file(env::var("LOG_FILE").expect("Log file must be set in ENV"), LevelFilter::Info).unwrap();
            //Example of Creating users
            match create_user(&pool, "John Doe", "john@example.com").await  
                {

                    Ok(_)=>{info!("Created test user sucessfully")},
                    Err(error)=>{error!("{error} occured please verify Postgres connection")}

                };

            let _user = match get_user(&pool, 1).await 
                {
                    Ok(x)=>{info!("Test user retreived successfully");x},
                    Err(error)=>{error!("{error} occured please verify Postgres connection");panic!("{error}")}

                };
            let select_from_table = match select_from_table(&pool, Tables::User, 1).await 
                {
                    Ok(x)=>{x},
                    Err(error)=>{error!("{error} occured please verify Postgres connection");panic!("{error}")}
                 };
            println!("User: {:?}",select_from_table);
            //Example of updating an Email
            match update_user_email(&pool, 1, "john.doe@example.com").await
                {
                    Ok(_)=>{info!("Test user updated successfully");},
                    Err(error)=>{error!("{error} occured please verify Postgres connection");panic!("{error}")}

                };


            //Example of Creating a task
            match create_task(&pool, 1, "Make Tacos".to_owned(), "Cook shells and meat and combine with cheese".to_owned(), Date::from_calendar_date(2026, time::Month::August, 30).ok().unwrap()).await
                {
                    Ok(_)=>{info!("Test user updated successfully");},
                    Err(error)=>{error!("{error} occured please verify Postgres connection");panic!("{error}")}

                };

            //Example of deleting a User
            match delete_user(&pool, 1).await
            {
                Ok(_)=>{info!("Test user updated successfully");},
                Err(error)=>{error!("{error} occured please verify Postgres connection");panic!("{error}")}

            };

            match delete_task(&pool, 1, "Make Tacos".to_owned(), "Cook shells and meat and combine with cheese".to_owned(), Date::from_calendar_date(2026, time::Month::August, 30).ok().unwrap()).await
                {
                    Ok(_)=>{info!("Test user updated successfully");},
                    Err(error)=>{error!("{error} occured please verify Postgres connection");panic!("{error}")}

                };
            





    }