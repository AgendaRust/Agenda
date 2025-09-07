use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::DatabaseConnection;
use chrono::{NaiveDate, Duration, Local, Datelike};

use crate::dto::reportDTO::*;
use crate::service::report_service::ReportService;

/// Endpoint de teste básico
#[get("/test")]
pub fn test_endpoint() -> Json<&'static str> {
    Json("✅ Sistema de relatórios funcionando!")
}

/// 📊 Endpoint unificado para relatórios
#[get("/reports/<report_type>?<start_date>&<end_date>&<user_id>")]
pub async fn generate_report(
    db: &State<DatabaseConnection>,
    report_type: String,
    start_date: Option<String>,
    end_date: Option<String>,
    user_id: Option<i32>,
) -> Result<Json<ReportResponseDto>, Json<String>> {
    // Parse do tipo de relatório
    let report_type = match report_type.to_lowercase().as_str() {
        "weekly" | "semanal" => ReportType::Weekly,
        "monthly" | "mensal" => ReportType::Monthly,
        "annual" | "anual" => ReportType::Annual,
        _ => return Err(Json("Tipo de relatório inválido. Use: weekly, monthly, annual".to_string())),
    };

    // Calcular datas automaticamente se não fornecidas
    let today = Local::now().date_naive();
    let (start, end) = match (&start_date, &end_date) {
        (Some(start_str), Some(end_str)) => {
            let start = NaiveDate::parse_from_str(start_str, "%Y-%m-%d")
                .map_err(|_| Json("Data de início inválida. Use YYYY-MM-DD".to_string()))?;
            let end = NaiveDate::parse_from_str(end_str, "%Y-%m-%d")
                .map_err(|_| Json("Data final inválida. Use YYYY-MM-DD".to_string()))?;
            (start, end)
        }
        _ => {
            // Calcular automaticamente baseado no tipo
            match report_type {
                ReportType::Weekly => {
                    let days_since_monday = today.weekday().num_days_from_monday() as i64;
                    let monday = today - Duration::days(days_since_monday);
                    let sunday = monday + Duration::days(6);
                    (monday, sunday)
                }
                ReportType::Monthly => {
                    let first_day = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
                        .ok_or_else(|| Json("Erro ao calcular primeiro dia do mês".to_string()))?;
                    let next_month = if today.month() == 12 {
                        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
                    } else {
                        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
                    }
                    .ok_or_else(|| Json("Erro ao calcular próximo mês".to_string()))?;
                    let last_day = next_month - Duration::days(1);
                    (first_day, last_day)
                }
                ReportType::Annual => {
                    let first_day = NaiveDate::from_ymd_opt(today.year(), 1, 1)
                        .ok_or_else(|| Json("Erro ao calcular primeiro dia do ano".to_string()))?;
                    let last_day = NaiveDate::from_ymd_opt(today.year(), 12, 31)
                        .ok_or_else(|| Json("Erro ao calcular último dia do ano".to_string()))?;
                    (first_day, last_day)
                }
            }
        }
    };

    let request = ReportRequestDto {
        report_type,
        start_date: start,
        end_date: end,
        user_id,
    };

    match ReportService::generate_report(db.inner(), request).await {
        Ok(report) => Ok(Json(report)),
        Err(e) => Err(Json(format!("Erro ao gerar relatório: {}", e))),
    }
}

/// 📅 Endpoint para relatório de período específico
#[get("/reports/weekly/<week_start>")]
pub async fn weekly_report(
    db: &State<DatabaseConnection>,
    week_start: String,
) -> Result<Json<ReportResponseDto>, Json<String>> {
    let start_date = NaiveDate::parse_from_str(&week_start, "%Y-%m-%d")
        .map_err(|_| Json("Formato de data inválido. Use YYYY-MM-DD".to_string()))?;
    
    let end_date = start_date + Duration::days(6);

    let request = ReportRequestDto {
        report_type: ReportType::Weekly,
        start_date,
        end_date,
        user_id: None,
    };

    match ReportService::generate_report(db.inner(), request).await {
        Ok(report) => Ok(Json(report)),
        Err(e) => Err(Json(format!("Erro ao gerar relatório semanal: {}", e))),
    }
}

/// 📅 Endpoint para relatório mensal
#[get("/reports/monthly/<year>/<month>")]
pub async fn monthly_report(
    db: &State<DatabaseConnection>,
    year: i32,
    month: u32,
) -> Result<Json<ReportResponseDto>, Json<String>> {
    let start_date = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| Json("Data inválida".to_string()))?;
    
    let end_date = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - Duration::days(1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - Duration::days(1)
    };

    let request = ReportRequestDto {
        report_type: ReportType::Monthly,
        start_date,
        end_date,
        user_id: None,
    };

    match ReportService::generate_report(db.inner(), request).await {
        Ok(report) => Ok(Json(report)),
        Err(e) => Err(Json(format!("Erro ao gerar relatório mensal: {}", e))),
    }
}

/// 📅 Endpoint para relatório anual
#[get("/reports/annual/<year>")]
pub async fn annual_report(
    db: &State<DatabaseConnection>,
    year: i32,
) -> Result<Json<ReportResponseDto>, Json<String>> {
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| Json("Ano inválido".to_string()))?;
    
    let end_date = NaiveDate::from_ymd_opt(year, 12, 31)
        .ok_or_else(|| Json("Ano inválido".to_string()))?;

    let request = ReportRequestDto {
        report_type: ReportType::Annual,
        start_date,
        end_date,
        user_id: None,
    };

    match ReportService::generate_report(db.inner(), request).await {
        Ok(report) => Ok(Json(report)),
        Err(e) => Err(Json(format!("Erro ao gerar relatório anual: {}", e))),
    }
}

/// 💡 Endpoint para obter sugestões de datas
#[get("/reports/suggestions?<report_type>")]
pub fn get_date_suggestions(report_type: String) -> Json<Vec<String>> {
    let today = Local::now().date_naive();
    let mut suggestions = Vec::new();

    match report_type.to_lowercase().as_str() {
        "weekly" | "semanal" => {
            for i in 0..8 {
                let week_start = today - Duration::weeks(i);
                let monday = week_start - Duration::days(week_start.weekday().num_days_from_monday() as i64);
                suggestions.push(monday.format("%Y-%m-%d").to_string());
            }
        }
        "monthly" | "mensal" => {
            for i in 0..12 {
                let current_month = today.month() as i32;
                let (year, month) = if current_month - i <= 0 {
                    let adjusted_month = 12 + (current_month - i);
                    (today.year() - 1, adjusted_month as u32)
                } else {
                    (today.year(), (current_month - i) as u32)
                };
                suggestions.push(format!("{}-{:02}", year, month));
            }
        }
        "annual" | "anual" => {
            for i in 0..5 {
                suggestions.push((today.year() - i).to_string());
            }
        }
        _ => suggestions.push("Tipo inválido".to_string()),
    }

    Json(suggestions)
}