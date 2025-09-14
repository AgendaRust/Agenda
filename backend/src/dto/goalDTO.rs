use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike, Duration};

#[derive(Serialize, Deserialize)]
pub struct GoalDto {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub goal_type: String, // "weekly", "monthly", "annual"
    pub start_date: Option<String>, // Data específica (opcional)
}

#[derive(Serialize, Deserialize)]
pub struct GoalResponseDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub goal_type: String,
    pub date_start: String,
    pub date_end: String,
    pub days_remaining: i64,
    pub progress_percentage: f32,
}

impl GoalDto {
    /// Calcula as datas de início e fim baseado no tipo de meta
    pub fn calculate_dates(&self) -> (NaiveDate, NaiveDate) {
        let base_date = if let Some(ref date_str) = self.start_date {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Local::now().naive_local().date())
        } else {
            chrono::Local::now().naive_local().date()
        };

        match self.goal_type.to_lowercase().as_str() {
            "weekly" | "semanal" => {
                // Encontra a segunda-feira da semana
                let days_from_monday = base_date.weekday().num_days_from_monday();
                let start_date = base_date - Duration::days(days_from_monday as i64);
                let end_date = start_date + Duration::days(6); // Domingo
                (start_date, end_date)
            },
            "monthly" | "mensal" => {
                // Do dia atual até o último dia do mês
                let start_date = base_date;
                let year = base_date.year();
                let month = base_date.month();
                
                // Último dia do mês
                let next_month = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
                };
                let end_date = next_month - Duration::days(1);
                (start_date, end_date)
            },
            "annual" | "anual" => {
                // Do dia atual até o final do ano
                let start_date = base_date;
                let end_date = NaiveDate::from_ymd_opt(base_date.year(), 12, 31).unwrap();
                (start_date, end_date)
            },
            _ => {
                // Padrão: meta mensal
                let start_date = base_date;
                let year = base_date.year();
                let month = base_date.month();
                
                let next_month = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
                };
                let end_date = next_month - Duration::days(1);
                (start_date, end_date)
            }
        }
    }
}