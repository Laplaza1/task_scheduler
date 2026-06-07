use time::Date;
use sqlx::{Any, PgPool};



#[derive(sqlx::FromRow,Debug)]
pub struct Tasks{
    pub id:i32,
    pub task: String,
    pub desciption:String,
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
    pub name: String,
    pub email: String,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tables {
    User,
    Tasks,
    CompletedTasks,

}

impl Tables {
    pub fn name(&self)->&'static str{
        match self {
            Tables::CompletedTasks=>"completed_tasks",
            Tables::Tasks=>"tasks",
            Tables::User=>"users",
        }
    }
}



pub async fn reset_users_table(pool: &PgPool){
    // Drop the table if it exists
    let _ = sqlx::query("DROP TABLE IF EXISTS users CASCADE;")
        .execute(pool)
        .await;

    let _ = sqlx::query("DROP TABLE IF EXISTS tasks CASCADE;")
        .execute(pool)
        .await;

    let _ = sqlx::query("DROP TABLE IF EXISTS completed_tasks CASCADE;")
        .execute(pool)
        .await;

    // Now create it with the correct schema
    let u = sqlx::query(
        "
        CREATE TABLE users (
            id              SERIAL PRIMARY KEY,
            name            TEXT NOT NULL,
            email           TEXT UNIQUE NOT NULL,
            created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "
    )
    .execute(pool)
    .await;


    let t = sqlx::query(
        "
        CREATE TABLE tasks (
            id              SERIAL PRIMARY KEY,
            user_id        INTEGER NOT NULL,
            task            TEXT NOT NULL,
            description      TEXT,
            due_date        DATE,
            created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            weight          INTEGER DEFAULT 100,
            CONSTRAINT valid_due_date
                CHECK (due_date > created_at),

            CONSTRAINT valid_user_id
                FOREIGN KEY (user_id)
                REFERENCES users(id)

        )"
    )
    .execute(pool)
    .await;

    

    let ct = sqlx::query(
        "
        CREATE TABLE completed_tasks (
            id              SERIAL PRIMARY KEY,
            task_id         INTEGER NOT NULL,
            date            DATE,
            completetor     TEXT NOT NULL,
            created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )")

    .execute(pool)
    .await;


    println!("Users:{:?}   Task: {:?} Completed Task: {:?}",u,t,ct)

}


pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn select_from_table(
    pool: &sqlx::PgPool,         
    table: Tables,
    limit: i32,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    println!("Table: {:?}",table);
    let query = format!(
        "SELECT * FROM {} LIMIT $1",
        table.name()   // Safe because it's from a controlled enum
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
    sqlx::query("INSERT INTO tasks (user_id,task,description,due_date) VALUES ($1, $2, $3,$4) ")
    .bind(user_id)
    .bind(task)
    .bind(description)
    .bind(due_date)
    .execute(pool)
    .await?;
    Ok(())


}