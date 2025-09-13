use crate::dto::reportDTO::*;
use chrono::{NaiveDate, Datelike, Duration, Utc, Weekday, Timelike, TimeZone};
use sea_orm::*;
use std::collections::HashMap;
use crate::entity::task;
use crate::entity::goal;

/// Serviço para geração de relatórios e estatísticas
pub struct ReportService {
    db: DatabaseConnection,
}

impl ReportService {
    /// Cria uma instância do ReportService
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Obtém estatísticas de tarefas para um ano específico
    pub async fn stats_year(
        &self,
        user_id: i32,
        year: i32,
    ) -> Result<StatsYearResponse, DbErr> {
        let start_date = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();

        // Total de tarefas do usuário no ano especificado
        let total_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .count(&self.db) // Adicionado &
            .await?;

        // Tarefas executadas no ano
        let executed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Concluída"))
            .count(&self.db) // Adicionado &
            .await?;

        let pendent_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Pendente"))
            .count(&self.db)
            .await?;

        let delayed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Adiada"))
            .count(&self.db)
            .await?;

        // Calcular porcentagem
        let percentage_tasks = self.calculate_percentage(
            executed_tasks.try_into().unwrap(),
            total_tasks.try_into().unwrap()
        );

        // Buscar os detalhes das tarefas executadas
        let executed_task_details = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::Status.eq("Concluída"))
            .filter(task::Column::CompleteDate.gte(start_date))
            .filter(task::Column::CompleteDate.lte(end_date))
            .all(&self.db) // Adicionado &
            .await?;

        // Inicializar os contadores
        let mut shift_counts: HashMap<&'static str, i32> = HashMap::new();
        let mut category_counts: HashMap<String, i32> = HashMap::new();
        let mut month_counts: HashMap<u32, i32> = HashMap::new();
        let mut week_counts: HashMap<u32, i32> = HashMap::new();

        // Processar todas as tarefas
        for task in &executed_task_details {
            self.count_shift(&mut shift_counts, task.complete_date.hour());

            // Contar categorias
            *category_counts.entry(task.category.clone()).or_insert(0) += 1;

            // Contar meses
            let month = task.complete_date.month();
            *month_counts.entry(month).or_insert(0) += 1;

            // Contar semanas
            let week = task.complete_date.iso_week().week();
            *week_counts.entry(week).or_insert(0) += 1;
        }

        // Resto do código permanece o mesmo...
        let most_productive_shift_tasks = self.find_most_productive_shift(shift_counts);
        let most_used_category_tasks = self.find_most_used_category(category_counts);
        let most_productive_month_tasks = self.find_most_productive_month(month_counts);
        let most_productive_week_tasks = self.find_most_productive_week(week_counts);
        let classification_tasks = self.classify_performance(percentage_tasks);

        Ok(StatsYearResponse {
            total_tasks: total_tasks as i64,
            executed_tasks: executed_tasks as i64,
            pendent_tasks: pendent_tasks as i64,
            delayed_tasks: delayed_tasks as i64,
            percentage_tasks,
            year,
            most_productive_shift_tasks,
            most_used_category_tasks,
            most_productive_month_tasks,
            most_productive_week_tasks,
            classification_tasks,
        })
    }

    /// Obtém estatísticas de tarefas para um mês específico
    pub async fn stats_month(
        &self,
        user_id: i32,
        year: i32,
        month: i32
    ) -> Result<StatsMonthResponse, DbErr> {
        let start_date = Utc.with_ymd_and_hms(year, month.try_into().unwrap(), 1, 0, 0, 0).unwrap();

        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let first_day_of_next_month = Utc.with_ymd_and_hms(next_year, next_month.try_into().unwrap(), 1, 0, 0, 0).unwrap();
        let end_date = first_day_of_next_month - Duration::seconds(1);

        // Total de tarefas do usuário no mês especificado
        let total_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .count(&self.db)
            .await?;

        // Tarefas executadas no mês
        let executed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Concluída"))
            .count(&self.db)
            .await?;

        let pendent_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Pendente"))
            .count(&self.db)
            .await?;

        let delayed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Adiada"))
            .count(&self.db)
            .await?;

        // Calcular porcentagem
        let percentage_tasks = self.calculate_percentage(
            executed_tasks.try_into().unwrap(),
            total_tasks.try_into().unwrap()
        );

        // Buscar os detalhes das tarefas executadas
        let executed_task_details = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::Status.eq("Concluída"))
            .filter(task::Column::CompleteDate.gte(start_date))
            .filter(task::Column::CompleteDate.lte(end_date))
            .all(&self.db)
            .await?;

        // Inicializar os contadores
        let mut shift_counts: HashMap<&'static str, i32> = HashMap::new();
        let mut category_counts: HashMap<String, i32> = HashMap::new();
        let mut week_counts: HashMap<u32, i32> = HashMap::new();

        // Processar todas as tarefas
        for task in &executed_task_details {
            self.count_shift(&mut shift_counts, task.complete_date.hour());

            // Contar categorias
            *category_counts.entry(task.category.clone()).or_insert(0) += 1;

            // Contar semanas
            let week = task.complete_date.iso_week().week();
            *week_counts.entry(week).or_insert(0) += 1;
        }

        // Encontrar estatísticas mais relevantes
        let most_productive_shift_tasks = self.find_most_productive_shift(shift_counts);
        let most_used_category_tasks = self.find_most_used_category(category_counts);
        let most_productive_week_tasks = self.find_most_productive_week(week_counts);
        let classification_tasks = self.classify_performance(percentage_tasks);

        Ok(StatsMonthResponse {
            total_tasks: total_tasks as i64,
            executed_tasks: executed_tasks as i64,
            pendent_tasks: pendent_tasks as i64,
            delayed_tasks: delayed_tasks as i64,
            percentage_tasks,
            year,
            month,
            most_productive_shift_tasks,
            most_used_category_tasks,
            most_productive_week_tasks,
            classification_tasks,
        })
    }

    /// Obtém estatísticas de tarefas para uma semana específica
    pub async fn stats_week(
        &self,
        user_id: i32,
        year: i32,
        week_num: i32,
    ) -> Result<StatsWeekResponse, DbErr> {
        // Calcula o primeiro dia da semana especificada
        let start_of_week_naive = NaiveDate::from_isoywd_opt(year, week_num.try_into().unwrap(), Weekday::Mon)
            .expect("Ano ou número de semana inválido.");
        let start_date = start_of_week_naive.and_hms_opt(0, 0, 0).unwrap().and_utc();

        // O fim da semana é o início da próxima semana menos 1 segundo
        let end_date = start_date + Duration::weeks(1) - Duration::seconds(1);

        // Total de tarefas do usuário na semana especificada
        let total_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .count(&self.db)
            .await?;

        // Tarefas executadas na semana
        let executed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Concluída"))
            .count(&self.db)
            .await?;

        let pendent_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Pendente"))
            .count(&self.db)
            .await?;

        let delayed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Adiada"))
            .count(&self.db)
            .await?;

        // Calcular porcentagem
        let percentage_tasks = self.calculate_percentage(
            executed_tasks.try_into().unwrap(),
            total_tasks.try_into().unwrap()
        );

        // Buscar os detalhes das tarefas executadas
        let executed_task_details = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::Status.eq("Concluída"))
            .filter(task::Column::CompleteDate.gte(start_date))
            .filter(task::Column::CompleteDate.lte(end_date))
            .all(&self.db)
            .await?;

        // Inicializar contadores
        let mut shift_counts: HashMap<&'static str, i32> = HashMap::new();
        let mut category_counts: HashMap<String, i32> = HashMap::new();
        let mut day_counts: HashMap<&'static str, i32> = HashMap::new();

        // Processar todas as tarefas
        for task in &executed_task_details {
            self.count_shift(&mut shift_counts, task.complete_date.hour());

            // Contar categorias
            *category_counts.entry(task.category.clone()).or_insert(0) += 1;

            // Contar dias da semana
            let day_name = self.get_weekday_name(task.complete_date.weekday());
            *day_counts.entry(day_name).or_insert(0) += 1;
        }

        // Encontrar estatísticas mais relevantes
        let most_productive_shift_tasks = self.find_most_productive_shift(shift_counts);
        let most_used_category_tasks = self.find_most_used_category(category_counts);
        let most_productive_day_tasks = self.find_most_productive_day(day_counts);
        let classification_tasks = self.classify_performance(percentage_tasks);

        Ok(StatsWeekResponse {
            total_tasks: total_tasks as i64,
            executed_tasks: executed_tasks as i64,
            pendent_tasks: pendent_tasks as i64,
            delayed_tasks: delayed_tasks as i64,
            percentage_tasks,
            year,
            week: week_num,
            most_productive_shift_tasks,
            most_used_category_tasks,
            most_productive_day_tasks,
            classification_tasks,
        })
    }

    // Métodos auxiliares

    /// Calcula a porcentagem de tarefas executadas
    fn calculate_percentage(&self, executed: i64, total: i64) -> f64 {
        if total > 0 {
            (executed as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Conta os turnos com base na hora
    fn count_shift(&self, shift_counts: &mut HashMap<&'static str, i32>, hour: u32) {
        let shift_name = match hour {
            6..=11 => "Manhã",
            12..=17 => "Tarde",
            18..=23 => "Noite",
            _ => "Madrugada", // 0..=5
        };
        *shift_counts.entry(shift_name).or_insert(0) += 1;
    }

    /// Retorna o nome do dia da semana
    fn get_weekday_name(&self, weekday: Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "Segunda-feira",
            Weekday::Tue => "Terça-feira",
            Weekday::Wed => "Quarta-feira",
            Weekday::Thu => "Quinta-feira",
            Weekday::Fri => "Sexta-feira",
            Weekday::Sat => "Sábado",
            Weekday::Sun => "Domingo",
        }
    }

    /// Encontra o turno mais produtivo
    fn find_most_productive_shift(&self, shift_counts: HashMap<&'static str, i32>) -> String {
        shift_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(shift_name, _)| shift_name.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    }

    /// Encontra a categoria mais usada
    fn find_most_used_category(&self, category_counts: HashMap<String, i32>) -> String {
        category_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(category, _)| category)
            .unwrap_or_else(|| "N/A".to_string())
    }

    /// Encontra o mês mais produtivo
    fn find_most_productive_month(&self, month_counts: HashMap<u32, i32>) -> String {
        month_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(month_num, _)| {
                match month_num {
                    1 => "Janeiro",
                    2 => "Fevereiro",
                    3 => "Março",
                    4 => "Abril",
                    5 => "Maio",
                    6 => "Junho",
                    7 => "Julho",
                    8 => "Agosto",
                    9 => "Setembro",
                    10 => "Outubro",
                    11 => "Novembro",
                    12 => "Dezembro",
                    _ => "Inválido",
                }.to_string()
            })
            .unwrap_or_else(|| "N/A".to_string())
    }

    /// Encontra a semana mais produtiva
    fn find_most_productive_week(&self, week_counts: HashMap<u32, i32>) -> String {
        week_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(week_num, _)| format!("Semana {}", week_num))
            .unwrap_or_else(|| "N/A".to_string())
    }

    /// Encontra o dia mais produtivo
    fn find_most_productive_day(&self, day_counts: HashMap<&'static str, i32>) -> String {
        day_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(day_name, _)| day_name.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    }

    /// Determina a classificação da performance baseada na porcentagem de tarefas executadas
    fn classify_performance(&self, percentage: f64) -> String {
        match percentage {
            p if p >= 100.0 => "Perfeito".to_string(),
            p if p > 80.0 => "Boa".to_string(),
            p if p > 60.0 => "Média".to_string(),
            p if p > 40.0 => "Ruim".to_string(),
            p if p > 20.0 => "Péssima".to_string(),
            _ => "Nem tentou".to_string(),
        }
    }
}