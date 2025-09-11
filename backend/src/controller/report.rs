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
        Ok((total, executed, percentage, year, shift, category, month, week)) => {
            Ok(Json(TaskStatsYearResponse {
                total_tasks: total,
                executed_tasks: executed,
                percentage,
                year,
                most_productive_shift: shift,
                most_used_category: category,
                most_productive_month: month,
                most_productive_week: week,
            }))
        },
        Err(_) => {
            // Em caso de erro, retorna valores padrão
            Ok(Json(TaskStatsYearResponse {
                total_tasks: 0,
                executed_tasks: 0,
                percentage: 0.0,
                year,
                most_productive_shift: "N/A".to_string(),
                most_used_category: "N/A".to_string(),
                most_productive_month: "N/A".to_string(),
                most_productive_week: "N/A".to_string(),
            }))
        }
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
        Ok((total, executed, percentage, year, month, shift, category, week)) => {
            Ok(Json(TaskStatsMonthResponse {
                total_tasks: total,
                executed_tasks: executed,
                percentage,
                year,
                month,
                most_productive_shift: shift,
                most_used_category: category,
                most_productive_week: week,
            }))
        },
        Err(_) => {
            // Em caso de erro, retorna valores padrão
            Ok(Json(TaskStatsMonthResponse {
                total_tasks: 0,
                executed_tasks: 0,
                percentage: 0.0,
                year,
                month,
                most_productive_shift: "N/A".to_string(),
                most_used_category: "N/A".to_string(),
                most_productive_week: "N/A".to_string(),
            }))
        }
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
        Ok((total, executed, percentage, year, week, shift, category, day)) => {
           Ok(Json(TaskStatsWeekResponse {
                total_tasks: total,
                executed_tasks: executed,
                percentage,
                year,
                week,
                most_productive_shift: shift,
                most_used_category: category,
                most_productive_day: day,
            }))
        },
        Err(_) => {
            // Em caso de erro, retorna valores padrão
            Ok(Json(TaskStatsWeekResponse {
                total_tasks: 0,
                executed_tasks: 0,
                percentage: 0.0,
                year,
                week,
                most_productive_shift: "N/A".to_string(),
                most_used_category: "N/A".to_string(),
                most_productive_day: "N/A".to_string(),
            }))
        }
    }
}
