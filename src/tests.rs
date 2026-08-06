use std::{fmt::format, net::TcpStream, panic, process::ExitCode, time::Duration};

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
pub async fn verify_normal_chars(x:&String)
    

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
            info!("Init postgres test");
            simple_logging::log_to_file(env::var("LOG_FILE").expect("Log file must be set in ENV"), LevelFilter::Info).unwrap();
            //Example of Creating users
            let _create_user = match create_user(&pool, "John Doe", "john@example.com").await  
                {

                    Ok(_)=>{info!("Created test user sucessfully")},
                    Err(error)=>{error!("{error} occured @ create_user please verify Postgres connection")}

                };

            let _user = match get_user(&pool, 1).await 
                {
                    Ok(x)=>{info!("Test user retreived successfully");x},
                    Err(error)=>{error!("{error} occured @ get_user please verify Postgres connection");std::process::exit(1)}

                };
            let select_from_table = match select_from_table(&pool, Tables::User, 1).await 
                {
                    Ok(x)=>{x},
                    Err(error)=>{error!("{error} occured @select_from_table please verify Postgres connection");std::process::exit(1)}
                 };
            
            //Example of updating an Email
            match update_user_email(&pool, 1, "john.doe@example.com").await
                {
                    Ok(_)=>{info!("Test user updated successfully");},
                    Err(error)=>{error!("{error} occured @ update_user_email please verify Postgres connection");std::process::exit(1)}

                };


            //Example of Creating a task
            match create_task(&pool, 1, "Make Tacos".to_owned(), "Cook shells and meat and combine with cheese".to_owned(), Date::from_calendar_date(2026, time::Month::August, 30).ok().unwrap()).await
                {
                    Ok(_)=>{info!("Test task created successfully");},
                    Err(error)=>{error!("{error} occured @ create_task please verify Postgres connection");std::process::exit(1)}

                };
            match delete_task(&pool, 1, "Make Tacos".to_owned(), "Cook shells and meat and combine with cheese".to_owned(), Date::from_calendar_date(2026, time::Month::August, 30).ok().unwrap()).await
                            {
                                Ok(_)=>{info!("Test take deleted successfully");},
                                Err(error)=>{error!("{error} occured @ delete_task please verify Postgres connection");std::process::exit(1)}

                            };
            
            //Example of deleting a User
            match delete_user(&pool, 1).await
            {
                Ok(_)=>{info!("Test user deleted successfully");},
                Err(error)=>{error!("{error} occured @ delete_user please verify Postgres connection");std::process::exit(1)}

            };

            





    }
    // port checker

///Port Checking function
/// 
/// Goal is to verify open ports and handle unexpected open ports
/// 
pub async fn port_checker(start:u32,stop:u32)->Result<String,Error> {

            let mut open_ports =1 as u32;
            for i in start..stop
                {
                    let host = match env::var("HOST")
                        {
                            Ok(x)=>{x},
                            Err(error)=>{warn!("{error} appeared while trying to find host. Please verify");error.to_string()}


                        };
                    let address = format!("{:?}:{}",host,i); 
                    let timeout = Duration::from_secs(9);
                    match TcpStream::connect_timeout(&address.parse().unwrap(), timeout) 
                        {
                            Ok(_) => {open_ports+=1;info!("Port {i} is open!")},
                            Err(error) => {warn!("Port {i} is Closed! by {error}")},
                        }
                    
                }
            Ok(format!("Port Check concluded there are {} open ports",open_ports).to_string())
}


pub async fn init(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Step 1: Reset users table
    reset_users_table(pool).await;

    // Step 2: Log after reset is done
    info!("DB connection established!");
    info!("Starting Tests & Examples");

    // Step 3: Run tests/examples
    postgres_init_test(pool).await;

    Ok(())
}

