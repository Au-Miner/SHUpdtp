use crate::database::{Pool, SyncMongo};
use crate::errors::ServiceError;
use crate::judge_actor::JudgeActorAddr;
use crate::models::contests::*;
use crate::models::users::LoggedUser;
use crate::services::contest;
use crate::services::region;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::*;

#[derive(Deserialize)]
pub struct CreateContestBody {
    region: String,
    title: String,
    introduction: Option<String>,
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
    seal_time: Option<NaiveDateTime>,
    settings: Option<ContestSettings>,
    password: Option<String>,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateContestBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let cur_user = logged_user.0.unwrap();
    if cur_user.role != "sup" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || {
        contest::create(
            body.region.clone(),
            body.title.clone(),
            body.introduction.clone(),
            body.start_time.clone(),
            body.end_time.clone(),
            body.seal_time.clone(),
            if let Some(settings) = body.settings.clone() {
                settings
            } else {
                ContestSettings::default()
            },
            body.password.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct GetContestListParams {
    title_filter: Option<String>,
    limit: i32,
    offset: i32,
}

#[get("")]
pub async fn get_contest_list(
    query: web::Query<GetContestListParams>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        contest::get_contest_list(
            query.title_filter.clone(),
            query.limit,
            query.offset,
            if let Some(user) = logged_user.0 {
                Some(user.id)
            } else {
                None
            },
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct InsertToContestBody {
    problem_ids: Vec<i32>,
}

#[post("/{region}")]
pub async fn insert_problems(
    web::Path(region): web::Path<String>,
    body: web::Json<InsertToContestBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let cur_user = logged_user.0.unwrap();
    if cur_user.role != "sup" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let res =
        web::block(move || region::insert_problems(region, body.problem_ids.clone(), None, pool))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                e
            })?;

    Ok(HttpResponse::Ok().json(&res))
}

/*
#[derive(Deserialize)]
pub struct GetContestColumnListParams {
    inner_id_filter: Option<i32>,
    problem_id_filter: Option<i32>,
    title_filter: Option<String>,
    tag_filter: Option<Vec<String>>,
    difficulty_filter: Option<String>,
    inner_id_order: Option<bool>,
    problem_id_order: Option<bool>,
    difficulty_order: Option<bool>,
    limit: i32,
    offset: i32,
}

#[get("/{region}")]
pub async fn get_item_list(
    web::Path(region): web::Path<String>,
    query: web::Query<GetContestColumnListParams>,
    pool: web::Data<Pool>,
    mongodb_database: web::Data<SyncMongo>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        contest::get_item_list(
            region,
            query.inner_id_filter,
            query.problem_id_filter,
            query.title_filter.clone(),
            query.tag_filter.clone(),
            query.difficulty_filter.clone(),
            query.inner_id_order.clone(),
            query.problem_id_order.clone(),
            query.difficulty_order.clone(),
            query.limit,
            query.offset,
            pool,
            mongodb_database,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/{region}/{inner_id}")]
pub async fn get_problem(
    web::Path((region, inner_id)): web::Path<(String, i32)>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || region::get_problem(region, inner_id, pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct CreateContestSubmissionBody {
    src: String,
    language: String,
}

#[post("/{region}/{inner_id}/submission")]
pub async fn create_submission(
    web::Path((region, inner_id)): web::Path<(String, i32)>,
    body: web::Json<CreateContestSubmissionBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
    judge_actor: web::Data<JudgeActorAddr>,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }

    let res = web::block(move || {
        region::create_submission(
            region,
            inner_id,
            logged_user.0.unwrap().id,
            body.src.clone(),
            body.language.clone(),
            pool,
            judge_actor,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}
*/