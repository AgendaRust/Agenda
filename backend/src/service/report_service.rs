use crate::dto::reportDTO::*;
use chrono::{NaiveDate, Datelike, Duration};
use sea_orm::*;
use std::collections::HashMap;

pub struct ReportService;

impl ReportService {
    /// Gera relatório unificado (semanal, mensal ou anual)
    pub async fn generate_report(
        db: &DatabaseConnection,
        request: ReportRequestDto,
    ) -> Result<ReportResponseDto, DbErr> {
        let period = Self::format_period(&request);
        
        let goal_summary: GoalSummary = Self::get_goal_summary(db, &request).await?;
        let task_summary = Self::get_task_summary(db, &request).await?;
        let productivity_insights = Self::get_productivity_insights(db, &request).await?;
        let category_analysis = Self::get_category_analysis(db, &request).await?;

        Ok(ReportResponseDto {
            period,
            report_type: request.report_type,
            goal_summary,
            task_summary,
            productivity_insights,
            category_analysis,
        })
    }

    /// Análise de metas com raw SQL - CORRETO (usando date_start)
    async fn get_goal_summary(
        db: &DatabaseConnection,
        request: &ReportRequestDto,
    ) -> Result<GoalSummary, DbErr> {
        let sql = r#"
            SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed,
                SUM(CASE WHEN status = 'PartiallyCompleted' THEN 1 ELSE 0 END) as partial,
                SUM(CASE WHEN status = 'NotCompleted' THEN 1 ELSE 0 END) as not_completed
            FROM goal 
            WHERE date_start BETWEEN ? AND ?
        "#;

        let result: Option<(i32, i32, i32, i32)> = db
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                sql,
                vec![
                    request.start_date.into(),
                    request.end_date.into(),
                ],
            ))
            .await?
            .and_then(|row| {
                Some((
                    row.try_get::<i32>("", "total").ok()?,
                    row.try_get::<i32>("", "completed").ok()?,
                    row.try_get::<i32>("", "partial").ok()?,
                    row.try_get::<i32>("", "not_completed").ok()?,
                ))
            });

        let (total, completed, partial, not_completed) = result.unwrap_or((0, 0, 0, 0));
        
        let completion_percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        let partial_completion_percentage = if total > 0 {
            (partial as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        Ok(GoalSummary {
            total_goals: total,
            completed_goals: completed,
            partially_completed_goals: partial,
            not_completed_goals: not_completed,
            completion_percentage,
            partial_completion_percentage,
        })
    }

    /// Análise de tarefas com raw SQL - CORRIGIDO (usando begin_date)
    async fn get_task_summary(
        db: &DatabaseConnection,
        request: &ReportRequestDto,
    ) -> Result<TaskSummary, DbErr> {
        let sql = r#"
            SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed,
                SUM(CASE WHEN status = 'PartiallyCompleted' THEN 1 ELSE 0 END) as partial,
                SUM(CASE WHEN status = 'Postponed' THEN 1 ELSE 0 END) as postponed
            FROM task 
            WHERE begin_date BETWEEN ? AND ?
        "#;

        let result: Option<(i32, i32, i32, i32)> = db
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                sql,
                vec![
                    request.start_date.into(),
                    request.end_date.into(),
                ],
            ))
            .await?
            .and_then(|row| {
                Some((
                    row.try_get::<i32>("", "total").ok()?,
                    row.try_get::<i32>("", "completed").ok()?,
                    row.try_get::<i32>("", "partial").ok()?,
                    row.try_get::<i32>("", "postponed").ok()?,
                ))
            });

        let (total, completed, partial, postponed) = result.unwrap_or((0, 0, 0, 0));
        
        let completion_percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        let partial_completion_percentage = if total > 0 {
            (partial as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        Ok(TaskSummary {
            total_tasks: total,
            completed_tasks: completed,
            partially_completed_tasks: partial,
            postponed_tasks: postponed,
            completion_percentage,
            partial_completion_percentage,
        })
    }

    /// Insights de produtividade com raw SQL - CORRIGIDO (usando begin_date)
    async fn get_productivity_insights(
        db: &DatabaseConnection,
        request: &ReportRequestDto,
    ) -> Result<ProductivityInsights, DbErr> {
        // Períodos mais produtivos baseados no tipo de relatório
        let period_sql: &'static str = match request.report_type {
            ReportType::Weekly => {
                r#"
                SELECT 
                    strftime('%Y-W%W', begin_date) as period,
                    SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed_tasks
                FROM task 
                WHERE begin_date BETWEEN ? AND ?
                GROUP BY strftime('%Y-W%W', begin_date)
                ORDER BY completed_tasks DESC
                LIMIT 5
                "#
            }
            ReportType::Monthly => {
                r#"
                SELECT 
                    strftime('%Y-%m', begin_date) as period,
                    SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed_tasks
                FROM task 
                WHERE begin_date BETWEEN ? AND ?
                GROUP BY strftime('%Y-%m', begin_date)
                ORDER BY completed_tasks DESC
                LIMIT 5
                "#
            }
            ReportType::Annual => {
                r#"
                SELECT 
                    strftime('%Y', begin_date) as period,
                    SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed_tasks
                FROM task 
                WHERE begin_date BETWEEN ? AND ?
                GROUP BY strftime('%Y', begin_date)
                ORDER BY completed_tasks DESC
                LIMIT 5
                "#
            }
        };

        let period_results = db
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                period_sql,
                vec![
                    request.start_date.into(),
                    request.end_date.into(),
                ],
            ))
            .await?;

        let most_productive_periods: Vec<ProductivePeriod> = period_results
            .iter()
            .filter_map(|row| {
                Some(ProductivePeriod {
                    period: row.try_get::<String>("", "period").ok()?,
                    completed_tasks: row.try_get::<i32>("", "completed_tasks").ok()?,
                    completed_goals: 0, // Implementar se necessário
                    total_score: row.try_get::<i32>("", "completed_tasks").ok()?,
                })
            })
            .collect();

        // Análise de turnos mais produtivos - CORRIGIDO (usando begin_date)
        let time_slot_sql = r#"
            SELECT 
                CASE 
                    WHEN CAST(strftime('%H', begin_date) AS INTEGER) BETWEEN 6 AND 11 THEN 'morning'
                    WHEN CAST(strftime('%H', begin_date) AS INTEGER) BETWEEN 12 AND 17 THEN 'afternoon'
                    ELSE 'evening'
                END as time_slot,
                COUNT(*) as total_tasks,
                SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed_tasks
            FROM task 
            WHERE begin_date BETWEEN ? AND ?
            GROUP BY time_slot
            ORDER BY completed_tasks DESC
        "#;

        let time_slot_results = db
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                time_slot_sql,
                vec![
                    request.start_date.into(),
                    request.end_date.into(),
                ],
            ))
            .await?;

        let most_productive_time_slots: Vec<TimeSlotProductivity> = time_slot_results
            .iter()
            .filter_map(|row| {
                let total = row.try_get::<i32>("", "total_tasks").ok()?;
                let completed = row.try_get::<i32>("", "completed_tasks").ok()?;
                let percentage = if total > 0 {
                    (completed as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                Some(TimeSlotProductivity {
                    time_slot: row.try_get::<String>("", "time_slot").ok()?,
                    completed_tasks: completed,
                    total_tasks: total,
                    productivity_percentage: percentage,
                })
            })
            .collect();

        Ok(ProductivityInsights {
            most_productive_periods,
            most_productive_time_slots,
        })
    }

    /// Análise de categorias com raw SQL - CORRIGIDO
    async fn get_category_analysis(
        db: &DatabaseConnection,
        request: &ReportRequestDto,
    ) -> Result<CategoryAnalysis, DbErr> {
        // Top categorias de tarefas - CORRIGIDO (usando begin_date)
        let task_categories_sql = r#"
            SELECT 
                category,
                COUNT(*) as total_items,
                SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed_items
            FROM task 
            WHERE begin_date BETWEEN ? AND ? AND category IS NOT NULL AND category != ''
            GROUP BY category
            ORDER BY completed_items DESC
            LIMIT 10
        "#;

        let task_results = db
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                task_categories_sql,
                vec![
                    request.start_date.into(),
                    request.end_date.into(),
                ],
            ))
            .await?;

        let top_task_categories: Vec<CategoryStats> = task_results
            .iter()
            .filter_map(|row| {
                let total = row.try_get::<i32>("", "total_items").ok()?;
                let completed = row.try_get::<i32>("", "completed_items").ok()?;
                let percentage = if total > 0 {
                    (completed as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                Some(CategoryStats {
                    category_name: row.try_get::<String>("", "category").ok()?,
                    total_items: total,
                    completed_items: completed,
                    completion_percentage: percentage,
                })
            })
            .collect();

        // Top categorias de metas - CORRETO (usando date_start)
        let goal_categories_sql = r#"
            SELECT 
                category,
                COUNT(*) as total_items,
                SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed_items
            FROM goal 
            WHERE date_start BETWEEN ? AND ? AND category IS NOT NULL AND category != ''
            GROUP BY category
            ORDER BY completed_items DESC
            LIMIT 10
        "#;

        let goal_results = db
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                goal_categories_sql,
                vec![
                    request.start_date.into(),
                    request.end_date.into(),
                ],
            ))
            .await?;

        let top_goal_categories: Vec<CategoryStats> = goal_results
            .iter()
            .filter_map(|row| {
                let total = row.try_get::<i32>("", "total_items").ok()?;
                let completed = row.try_get::<i32>("", "completed_items").ok()?;
                let percentage = if total > 0 {
                    (completed as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                Some(CategoryStats {
                    category_name: row.try_get::<String>("", "category").ok()?,
                    total_items: total,
                    completed_items: completed,
                    completion_percentage: percentage,
                })
            })
            .collect();

        Ok(CategoryAnalysis {
            top_task_categories,
            top_goal_categories,
        })
    }

    /// Formatar período para exibição
    fn format_period(request: &ReportRequestDto) -> String {
        match request.report_type {
            ReportType::Weekly => {
                format!(
                    "Semana de {} a {}",
                    request.start_date.format("%d/%m/%Y"),
                    request.end_date.format("%d/%m/%Y")
                )
            }
            ReportType::Monthly => {
                format!(
                    "{}/{}",
                    request.start_date.format("%m"),
                    request.start_date.format("%Y")
                )
            }
            ReportType::Annual => request.start_date.format("%Y").to_string(),
        }
    }
}