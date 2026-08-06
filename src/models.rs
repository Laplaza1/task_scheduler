use std::{env, format, fs, path::PathBuf, println};

use log::{LevelFilter, error, info, warn};
use serde::{Deserialize, Serialize};
use time::Date;
use sqlx::{query_file, query_file_as, PgPool};
use crate::tests::{self, verify_normal_chars};


#[derive(sqlx::FromRow,Debug,Serialize,Deserialize)]
pub struct Tasks{
    pub id:i32,
    pub user_id:i32,
    pub task: String,
    pub description:String,
    pub due_date:Date,
    pub created_at:Date,
    pub updated_at:Date
}


#[derive(sqlx::FromRow,Debug)]
pub struct CompletedTasks{

            id:i32,
            task:String,
            date:Date,
            completetor:String,
            created_at:Date,
            updated_at:Date


}




#[derive(sqlx::FromRow,Debug)]
pub struct User {
    pub id: i32,
    pub _name: String,
    pub email: String,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tables {
    User,
    Tasks,
    CompletedTasks,

}

impl Tables {
    pub fn _name(&self)->&'static str{
        match self {
            Tables::CompletedTasks=>"completed_tasks",
            Tables::Tasks=>"tasks",
            Tables::User=>"users",
        }
    }
}



pub async fn reset_users_table(pool: &PgPool){
    simple_logging::log_to_file(env::var("LOG_FILE").expect("Log file must be set in ENV"), LevelFilter::Info).unwrap();
    // Drop the table if it exists
    info!("Starting reseting user table");
    
    let _ = sqlx::query("DROP TABLE IF EXISTS users CASCADE;")
        .execute(pool)
        .await;
    info!("Users sucessfully dropped");
    
    let _ = sqlx::query("DROP TABLE IF EXISTS tasks CASCADE;")
        .execute(pool)
        .await;

    info!("Tasks sucessfully dropped");
    
    let _ = sqlx::query("DROP TABLE IF EXISTS completed_tasks CASCADE;")
        .execute(pool)
        .await;
    info!("Completed Tasks sucessfully dropped");

    let directory_path = match env::current_dir()
                        {
                            Ok(y)=>
                                {
                                  y
                                    
                                        
                                },
                            _=>{std::process::exit(1)}

                        };
    
    
    let  init_users_path =  directory_path.clone().join(r"sql\init_users_table.sql");
    info!("{:?}",init_users_path);
    let init_users_str = &fs::read_to_string(&init_users_path);
    let init_users_str = match init_users_str 
        {
            Ok(x)=>{x},
            _=>{error!("Couldn't read init_users_sql item");std::process::exit(1)}

        };
    
                                            
    
    
    let init_tasks_path =  directory_path.clone().join(r"sql\init_tasks_table.sql");
    let init_tasks_str = &fs::read_to_string(&init_tasks_path);
     let init_tasks_str = match init_tasks_str 
        {
            Ok(x)=>{x},
            _=>{error!("Couldn't read init_users_sql item");std::process::exit(1)}

        };                                    

    
    let init_completed_tasks_path =  directory_path.clone().join(r"sql\init_completed_table.sql");
    let init_completed_tasks_str = &fs::read_to_string(&init_completed_tasks_path);
    let init_completed_tasks_str = match init_completed_tasks_str 
        {
            Ok(x)=>{x},
            _=>{error!("Couldn't read init_users_sql item");std::process::exit(1)}

        };
    

    info!("Creating Users table");
    let init_users_table= sqlx::query(&init_users_str)
    .execute(pool)
    .await;
    

    
    
    info!("Creating completed_tasks table");
    let init_completed_tasks_table = sqlx::query(
        init_completed_tasks_str)

    .execute(pool)
    .await;
    

    info!("Creating task table");
    let init_task_table = sqlx::query(
        init_tasks_str
    )
    .execute(pool)
    .await;


   info!(" Tables created Users:{} Task: {} Completed Task: {}",init_users_table.is_ok(),init_task_table.is_ok(),init_completed_tasks_table.is_ok())

}


pub async fn create_user(pool: &sqlx::PgPool, _name: &str, email: &str) -> Result<(), sqlx::Error> {
    verify_normal_chars(&_name.to_string()).await;
    let _= sqlx::query("INSERT INTO users (_name, email) VALUES ($1, $2)")
        .bind(_name)
        .bind(email)
        .execute(pool)
        .await;
    Ok(())
}


pub async fn select_from_table(
    pool: &sqlx::PgPool,         
    table: Tables,
    limit: i32,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let query = format!(
        "SELECT * FROM {} LIMIT $1",
        table._name()   // Safe because it's from a controlled enum
    );

    sqlx::query(&query)
        .bind(limit)
        .fetch_all(pool)
        .await
}


pub async fn get_user(pool: &sqlx::PgPool, user_id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}


pub async fn update_user_email(pool: &sqlx::PgPool, user_id: i32, new_email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET email = $1 WHERE id = $2")
        .bind(new_email)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_user(pool: &sqlx::PgPool, user_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}



pub async fn create_task(pool: &sqlx::PgPool,user_id: i32,task:String,description:String,due_date:Date)-> Result<(),sqlx::Error>{

    verify_normal_chars(&task).await;
    verify_normal_chars(&description).await;

    sqlx::query("INSERT INTO tasks (user_id,task,description,due_date) VALUES ($1, $2, $3,$4) ")
    .bind(user_id)
    .bind(task)
    .bind(description)
    .bind(due_date)
    .execute(pool)
    .await?;
    Ok(())


}

pub async fn delete_task(pool: &sqlx::PgPool,user_id: i32,task:String,description:String,due_date:Date)-> Result<(),sqlx::Error>{

    verify_normal_chars(&task).await;
    verify_normal_chars(&description).await;

    sqlx::query("DELETE FROM tasks
WHERE user_id = $1
  AND task = $2
  AND description = $3
  AND due_date = $4;")
    .bind(user_id)
    .bind(task)
    .bind(description)
    .bind(due_date)
    .execute(pool)
    .await?;
    Ok(())


}


pub async fn grab_task(pool: &sqlx::PgPool,user_id: i32)-> Result<Vec<Tasks>,sqlx::Error>{
    let x:Vec<Tasks>= sqlx::query_as::<_,Tasks>("select * from tasks where user_id = $1").bind(user_id).fetch_all(pool).await?;
        
    

    return Ok(x)

}