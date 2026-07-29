

use axum::{
    body::Body, debug_handler, extract::{ws::close_code::STATUS, Path, State}, http::{header::{self, COOKIE, SET_COOKIE}, HeaderMap, HeaderValue, Method, Response, StatusCode}, response::{self, IntoResponse, Json}, routing::{delete, get, post, put}, Router
};
use serde_json::Value;
use sqlx::Postgres;
use core::panic;
use std::{any::{type_name, type_name_of_val}, collections::HashMap, fmt::LowerHex, hash::{DefaultHasher, Hash, Hasher}, time::{Duration, SystemTime}};
use axum_extra::extract::{cookie,CookieJar};
use axum_governor::GovernorLayer;
use ::cookie::{Cookie, Expiration, SameSite};
use reqwest;
use tower_http::cors::{CorsLayer, AllowOrigin,Any};
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt, io::Cursor};
use time::OffsetDateTime;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool, Pool};

use crate::models::{*};


#[derive(Clone)]
pub struct AppPool {
    pub(crate) pool:Pool<Postgres>
}



pub async fn user_(State(poolState): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    

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
        )
            .await
            .unwrap();

    return StatusCode::ACCEPTED.into_response()


}



pub async fn get_task(State(poolState): State<AppPool>,headers:HeaderMap)->Response<Body>{
    

    let user_id = CookieJar::from_headers(&headers).get("user_id").map(|cookie| cookie.value().to_owned()).unwrap();
    
    



     let task = grab_task(&poolState.pool, user_id.parse().unwrap()).await;

    return StatusCode::ACCEPTED.into_response()








}


pub async fn task_(State(poolState): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    

    let user_id = match payload.get("user_id").unwrap() {
        Value::Number(x)=>{x.as_i64().unwrap() as i32},

        _=>{return StatusCode::NOT_ACCEPTABLE.into_response()}


    };
    
    let task = payload.get("task").unwrap();

    let description = payload.get("description").unwrap();

    let due_date = match payload.get("due_date") {

        Some(Value::Array(x)) =>{x[0].as_number().unwrap().as_i128()},

        Some(Value::Number(x)) =>{x.as_i128()},

        _ =>{None}
        
    };



    create_task(&poolState.pool, user_id, task.to_string(), description.to_string(), OffsetDateTime::from_unix_timestamp_nanos(due_date.unwrap()).unwrap().date()).await;

    return StatusCode::ACCEPTED.into_response()








}
pub async fn get_users(headerMap:HeaderMap,State(poolState): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{

    let jar = CookieJar::from_headers(&headerMap);
    match jar.get("GID"){

        Some(x)=>{x;},

        _=>{return StatusCode::BAD_REQUEST.into_response();}
    }
    
    return StatusCode::ACCEPTED.into_response()

}



pub async fn login(headerMap:HeaderMap,State(poolState): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{


    get_user(
        &poolState.pool,
match payload.get("user_id")
            {
                Some(Value::Number(x))=>{x.as_i64().unwrap() as i32},
                _=>{0}
            }
            
            )
                .await
                .expect("Error getting user");


            let mut newHeader = HeaderMap::new();
            
            let expires_in = Duration::from_secs(7 * 24 * 60 * 60);/// Days * Hours * Mins * Secs
            let expires_at = SystemTime::now() + expires_in;
            
            let mut cookier = (Cookie::new("GID", "placeholder"));
                cookier.set_expires(Expiration::DateTime(expires_at.into()));
                cookier.set_secure(true);
                cookier.set_same_site(SameSite::None);
                cookier.set_path("/");
            
            newHeader.append(SET_COOKIE, cookier.to_string().parse().unwrap());

            
            let x = (StatusCode::ACCEPTED,newHeader).into_response();

            

            return x

}




