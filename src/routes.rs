

use axum::{
    body::Body,extract::{ State}, http::{header::{ SET_COOKIE}, HeaderMap, HeaderValue, Response, StatusCode}, response::{IntoResponse, Json}
};
use serde_json::{ Value, json};
use sqlx::Postgres;

use std::{time::{Duration, SystemTime}};
use axum_extra::extract::{CookieJar};

use ::cookie::{Cookie, Expiration, SameSite};



use axum::http::Uri;
use time::OffsetDateTime;
use crate::{Auth::*, LimitState};
use axum_limit::Quota;
use sqlx::{Pool};
use log::{*};
use crate::models::{*};
use simple_logging::log_to_file;






#[derive(Clone)]
pub struct AppPool {
    pub(crate) pool:Pool<Postgres>,
    pub(crate) limits: LimitState<Uri>,
    pub(crate) api_quota: Quota,
}



pub async fn user_(State(pool_state): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    

    let name = match payload
            .get("name")
            {
                Some(Value::String(x))=>{x},
                _=>{error!("User ID  Data type is incorrect");return StatusCode::BAD_REQUEST.into_response()}
            };
    
    let email =match payload
            .get("email")
            {
                Some(Value::String(x))=>{x},
                _=>{error!("Email data type is incorrect");return StatusCode::BAD_REQUEST.into_response()}
            };
        


    create_user(
            &pool_state.pool,
            name,
            email
            )
                .await
                .ok();
                    

    return StatusCode::ACCEPTED.into_response()


}




pub async fn get_task(State(pool_state): State<AppPool>,headers:HeaderMap)->Response<Body>{


    let _ = log_to_file("app.log", log::LevelFilter::Info); 

    let user_id= match CookieJar::from_headers(&headers).get("user_id").map(|cookie| cookie.value().to_owned())
        {
            Some(x)=>{ match x.parse::<i32>()
                                {
                                    Ok(converted_x)=>{converted_x},
                                    _=>{
                                        log::error!("Couldn't convert {x} into i32");
                                        return StatusCode::BAD_REQUEST.into_response()
                                        }

                                }
                             },
            _ =>{
                log::error!("User ID is incorrect");
                return StatusCode::NOT_FOUND.into_response()
                }

        };

    let task = match grab_task(&pool_state.pool, user_id).await.ok()
                                   {
                                    Some(x)=>{x},
                                    _=>{
                                        log::error!("Tasks failed to fetch!");
                                        return StatusCode::NOT_FOUND.into_response()
                                        }


                                   };
    
    return Json(json!({"tasks":task})).into_response()
}


pub async fn task_(State(pool_state): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    

    let user_id = match payload.get("user_id") {
        Some(Value::Number(x))=>{
                                            match x.as_i64() 
                                                {
                                                    Some(x)=>{
                                                                    x as i32
                                                                  },
                                                    _=>{
                                                        log::error!("due date was incorrect!");
                                                        return StatusCode::NOT_ACCEPTABLE.into_response()
                                                        }
                                                }
                                        },

        _=>{
            log::error!("User ID is incorrect!");
            return StatusCode::NOT_ACCEPTABLE.into_response()
            }
    };
    
    let task = match payload.get("task") {
        Some(Value::String(x))=>{x},
        _=>{
            log::error!("due date data type was incorrect!");
            return StatusCode::NOT_ACCEPTABLE.into_response()
            }
    };

    let description:&String = match payload.get("description"){
        Some(Value::String(x))=>{x},
        _=>{
            log::error!("description data type was incorrect!");
            return StatusCode::NOT_ACCEPTABLE.into_response()
         }
    };
    let due_date:i128 = match payload.get("due_date") {

        Some(Value::Array(x)) =>{
                                            match x[0].as_number()
                                                {
                                                    Some(x)=>{
                                                                                        match x.as_i128()   
                                                                                        {
                                                                                            Some(x)=>{x},
                                                                                            _=>{
                                                                                                log::error!("due date Vec inner data type was incorrect!");
                                                                                                return StatusCode::NOT_ACCEPTABLE.into_response()
                                                                                                }
                                                                                        }
                                                                                    },
                                                    _=>{
                                                        log::error!("due date data type was incorrect!");
                                                        return StatusCode::NOT_ACCEPTABLE.into_response()
                                                        }
                                                }
                                            },

        Some(Value::Number(x)) =>{
                                            match x.as_i128()
                                                {   
                                                    Some(x)=>{x},
                                                    _=>{
                                                        log::error!("due date data type was incorrect!");
                                                        return StatusCode::NOT_ACCEPTABLE.into_response()
                                                        }
                                                }
                                          },
        _ =>{
            log::error!("due date data type was an incorrect type!");
            return StatusCode::NOT_ACCEPTABLE.into_response()
            }
    };
    
    let created_task = create_task(
                                                        &pool_state.pool, 
                                                        user_id, 
                                                        task.to_string(), 
                                                        description.to_string(), 
                                            match OffsetDateTime::from_unix_timestamp_nanos(due_date).ok()
                                                            {
                                                                Some(x)=>{x.date()},
                                                                _=>{
                                                                    log::error!("offset date was incorrect!");
                                                                    return StatusCode::NOT_ACCEPTABLE.into_response()
                                                                }
                                                            }
                                                    ).await;
    if created_task.is_ok() {
        log::info!("Task {task} was created!");
    }
    return StatusCode::ACCEPTED.into_response()








}
pub async fn get_users(header_map:HeaderMap,State(pool_state): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{

    let jar = CookieJar::from_headers(&header_map);
    let _cookie = match jar.get("GID")
            {

                Some(x)=>{x},

                _=>{return StatusCode::BAD_REQUEST.into_response();}
            };
            

    
    return StatusCode::ACCEPTED.into_response()

}



pub async fn login(header_map:HeaderMap,State(pool_state): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>{


    let x =match get_user(
        &pool_state.pool,
match payload.get("user_id")
            {
                Some(Value::Number(x))=>
                    {
                        match x.as_i64()
                            {
                            Some(x)=>{x as i32},
                            _=>{
                                log::error!("ID conversion to i32 failed! given var was {x}");
                                return StatusCode::NOT_ACCEPTABLE.into_response()
                            }
                            }
                    }
                _=>{0}
                    
            }
            )
                .await
                {
                    Ok(x)=>{x},
                    Err(error)=>{error!("{error}@ geting user from get_user @ login function");std::process::exit(1)}

                };


            let mut new_header = HeaderMap::new();
            
            let expires_in = Duration::from_secs(7 * 24 * 60 * 60);// Days * Hours * Mins * Secs
            let expires_at = SystemTime::now() + expires_in;
            create_token(x.id);
            let mut cookier = (Cookie::new("GID", "placeholder"));
                cookier.set_expires(Expiration::DateTime(expires_at.into()));
                cookier.set_secure(true);
                cookier.set_same_site(SameSite::None);
                cookier.set_path("/");
            
            new_header.append(SET_COOKIE, match cookier.to_string().parse::<HeaderValue>().ok(){
                                                                                   Some(x)=>{x},
                                                                                    _=>{
                                                                                        log::error!("cookie failed to parse!");
                                                                                        return StatusCode::NOT_ACCEPTABLE.into_response()
                                                                                        }
                                                                                      
                                                                                      
                                                                                      });

            
            let x = (StatusCode::ACCEPTED,new_header).into_response();

            

            return x

}



pub async  fn delete_task_(State(pool_state): State<AppPool>,Json(payload): Json<serde_json::Value>)->Response<Body>
    {





        StatusCode::ACCEPTED.into_response()
    }
