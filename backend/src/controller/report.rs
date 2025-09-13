use rocket::serde::json::Json;
use rocket::{get, State};
use rocket::http::Status;
use sea_orm::DatabaseConnection;
use chrono::{NaiveDate, Duration, Local, Datelike};
use crate::controller::auth::UserClaim;
use crate::dto::reportDTO::*;
use crate::service::report_service::ReportService;

/// Endpoint para obter estatísticas de tarefas por ano
#[get("/stats/year/<year>")]
pub async fn get_tasks_stats_year(
    token: UserClaim,
    year: i32,
    db: &State<DatabaseConnection>,
) -> Result<Json<TaskStatsYearResponse>, (Status, String)>{
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    let report_service = ReportService::new(db.inner().clone());
    let result = report_service.tasks_stats_year(user_id, year).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Ok(Json(TaskStatsYearResponse::default_for_year(year)))
    }
}

/// Endpoint para obter estatísticas de tarefas por mês
#[get("/stats/month/<year>/<month>")]
pub async fn get_tasks_stats_month(
    token: UserClaim,
    year: i32,
    month: i32,
    db: &State<DatabaseConnection>,
) -> Result<Json<TaskStatsMonthResponse>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    let report_service = ReportService::new(db.inner().clone());
    let result = report_service.tasks_stats_month(user_id, year, month).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Ok(Json(TaskStatsMonthResponse::default_for_month(year, month)))
    }
}

/// Endpoint para obter estatísticas de tarefas por semana
#[get("/stats/week/<year>/<week>")]
pub async fn get_tasks_stats_week(
    token: UserClaim,
    year: i32,
    week: i32,
    db: &State<DatabaseConnection>,
) -> Result<Json<TaskStatsWeekResponse>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    let report_service = ReportService::new(db.inner().clone());
    let result = report_service.tasks_stats_week(user_id, year, week).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Ok(Json(TaskStatsWeekResponse::default_for_week(year, week)))
    }
}

