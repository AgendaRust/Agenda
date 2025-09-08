use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set, PaginatorTrait};
use chrono::{Timelike, Utc, TimeZone, Duration, Weekday, NaiveDate, Datelike};
use std::collections::HashMap;
use crate::dto::taskDTO::TaskDto;
use crate::entity::task;

pub struct TaskRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TaskRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<task::Model>, DbErr> {
        task::Entity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<task::Model>, DbErr> {
        task::Entity::find_by_id(id).one(self.db).await
    }

    pub async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<task::Model>, DbErr> {
        task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .all(self.db)
            .await
    }


    pub async fn tasks_stats_year(
        &self,
        user_id: i32,
        year: i32,
    ) -> Result<(i64, i64, f64, i32, String, String, String, String), DbErr> {
        let start_date = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();

        // Total de tarefas do usuário no ano especificado
        let total_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .count(self.db)
            .await?;

        // Tarefas executadas no ano
        let executed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Executada"))
            .count(self.db)
            .await?;

        // Calcular porcentagem
        let percentage = if total_tasks > 0 {
            (executed_tasks as f64 / total_tasks as f64) * 100.0
        } else {
            0.0
        };

        // 1. Buscar os detalhes das tarefas executadas
        let executed_task_details = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::Status.eq("Executada"))
            .filter(task::Column::CompleteDate.gte(start_date))
            .filter(task::Column::CompleteDate.lte(end_date))
            .all(self.db)
            .await?;

        // 2. Inicializar os contadores
        let mut shift_counts: HashMap<&'static str, i32> = HashMap::new();
        let mut category_counts: HashMap<String, i32> = HashMap::new();
        // NOVO: Contadores para mês e semana
        let mut month_counts: HashMap<u32, i32> = HashMap::new();
        let mut week_counts: HashMap<u32, i32> = HashMap::new();

        // 3. Iterar sobre as tarefas para contar tudo
        for task in &executed_task_details {
            // Lógica para contar o turno
            let hour = task.complete_date.hour();
            let shift_name = match hour {
                6..=11 => Some("Manhã"),
                12..=17 => Some("Tarde"),
                18..=23 => Some("Noite"),
                0..=5 => Some("Madrugada"),
                _ => None,
            };
            if let Some(name) = shift_name {
                *shift_counts.entry(name).or_insert(0) += 1;
            }

            // Lógica para contar as categorias
            *category_counts.entry(task.category.clone()).or_insert(0) += 1;

            // NOVO: Lógica para contar os meses
            let month = task.complete_date.month();
            *month_counts.entry(month).or_insert(0) += 1;

            // NOVO: Lógica para contar as semanas (usando o padrão ISO 8601)
            let week = task.complete_date.iso_week().week();
            *week_counts.entry(week).or_insert(0) += 1;
        }

        // 4. Encontrar o turno mais produtivo
        let most_productive_shift = shift_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(shift_name, _)| shift_name.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        // 5. Encontrar a categoria mais usada
        let most_used_category = category_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(category, _)| category)
            .unwrap_or_else(|| "N/A".to_string());

        // NOVO: 6. Encontrar o mês mais produtivo
        let most_productive_month = month_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(month_num, _)| {
                match month_num {
                    1 => "Janeiro", 2 => "Fevereiro", 3 => "Março", 4 => "Abril",
                    5 => "Maio", 6 => "Junho", 7 => "Julho", 8 => "Agosto",
                    9 => "Setembro", 10 => "Outubro", 11 => "Novembro", 12 => "Dezembro",
                    _ => "Inválido",
                }.to_string()
            })
            .unwrap_or_else(|| "N/A".to_string());

        // NOVO: 7. Encontrar a semana mais produtiva
        let most_productive_week = week_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(week_num, _)| format!("Semana {}", week_num))
            .unwrap_or_else(|| "N/A".to_string());


        // 8. Retornar todos os dados
        Ok((
            total_tasks as i64,
            executed_tasks as i64,
            percentage,
            year,
            most_productive_shift,
            most_used_category,
            most_productive_month, // <- Novo
            most_productive_week,   // <- Novo
        ))
    }

    pub async fn tasks_stats_month(&self, user_id: i32, year: i32, month: i32) -> Result<(i64, i64, f64, i32, i32, String, String, String), DbErr> {

        let start_date = Utc.with_ymd_and_hms(year, month.try_into().unwrap(), 1, 0, 0, 0).unwrap();

        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let first_day_of_next_month = Utc.with_ymd_and_hms(next_year, next_month.try_into().unwrap(), 1, 0, 0, 0).unwrap();
        let end_date = first_day_of_next_month - Duration::seconds(1);

        // Total de tarefas do usuário no ano especificado
        let total_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .count(self.db)
            .await?;

        // Tarefas executadas no ano
        let executed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Executada"))
            .count(self.db)
            .await?;

        // Calcular porcentagem
        let percentage = if total_tasks > 0 {
            (executed_tasks as f64 / total_tasks as f64) * 100.0
        } else {
            0.0
        };

        // 1. Buscar os detalhes das tarefas executadas
        let executed_task_details = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::Status.eq("Executada"))
            .filter(task::Column::CompleteDate.gte(start_date))
            .filter(task::Column::CompleteDate.lte(end_date))
            .all(self.db)
            .await?;

        // 2. Inicializar os contadores para turnos e categorias
        let mut shift_counts: HashMap<&'static str, i32> = HashMap::new();
        let mut category_counts: HashMap<String, i32> = HashMap::new();
        let mut week_counts: HashMap<u32, i32> = HashMap::new();


        // 3. Iterar sobre as tarefas para contar turnos E categorias
        for task in &executed_task_details {
            // Lógica para contar o turno (sem alterações)
            let hour = task.complete_date.hour();
            let shift_name = match hour {
                6..=11 => Some("Manhã"),
                12..=17 => Some("Tarde"),
                18..=23 => Some("Noite"),
                0..=5 => Some("Madrugada"),
                _ => None,
            };
            if let Some(name) = shift_name {
                *shift_counts.entry(name).or_insert(0) += 1;
            }

            // NOVO: Lógica para contar as categorias
            *category_counts.entry(task.category.clone()).or_insert(0) += 1;

            let week = task.complete_date.iso_week().week();
            *week_counts.entry(week).or_insert(0) += 1;
        }

        // 4. Encontrar o turno mais produtivo
        let most_productive_shift = shift_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(shift_name, _)| shift_name.to_string())
            .unwrap_or_else(|| "N/A".to_string());


        // 5. Encontrar a categoria com a maior contagem diretamente
        let most_used_category = category_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(category, _)| category)
            .unwrap_or_else(|| "N/A".to_string());

        let most_productive_week = week_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(week_num, _)| format!("Semana {}", week_num))
            .unwrap_or_else(|| "N/A".to_string());

        Ok((
            total_tasks as i64,
            executed_tasks as i64,
            percentage,
            year,
            month,
            most_productive_shift,
            most_used_category,
            most_productive_week,
        ))
    }

    pub async fn tasks_stats_week(
        &self,
        user_id: i32,
        year: i32,
        week_num: i32,
    ) -> Result<(i64, i64, f64, i32, i32, String, String, String), DbErr> {

        // Calcula o primeiro dia (segunda-feira) da semana especificada
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
            .count(self.db)
            .await?;

        // Tarefas executadas na semana
        let executed_tasks = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::BeginDate.gte(start_date))
            .filter(task::Column::BeginDate.lte(end_date))
            .filter(task::Column::Status.eq("Executada"))
            .count(self.db)
            .await?;

        // Calcular porcentagem
        let percentage = if total_tasks > 0 {
            (executed_tasks as f64 / total_tasks as f64) * 100.0
        } else {
            0.0
        };

        // 1. Buscar os detalhes das tarefas executadas no intervalo da semana
        let executed_task_details = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(task::Column::Status.eq("Executada"))
            .filter(task::Column::CompleteDate.gte(start_date))
            .filter(task::Column::CompleteDate.lte(end_date))
            .all(self.db)
            .await?;

        // 2. Inicializar contadores
        let mut shift_counts: HashMap<&'static str, i32> = HashMap::new();
        let mut category_counts: HashMap<String, i32> = HashMap::new();
        let mut day_counts: HashMap<&'static str, i32> = HashMap::new(); // NOVO: para o dia mais produtivo

        // 3. Iterar sobre as tarefas para contar turnos, categorias e dias
        for task in &executed_task_details {
            // Lógica para contar o turno (sem alterações)
            let hour = task.complete_date.hour();
            let shift_name = match hour {
                6..=11 => "Manhã",
                12..=17 => "Tarde",
                18..=23 => "Noite",
                _ => "Madrugada", // 0..=5
            };
            *shift_counts.entry(shift_name).or_insert(0) += 1;

            // Lógica para contar as categorias (sem alterações)
            *category_counts.entry(task.category.clone()).or_insert(0) += 1;

            // NOVO: Lógica para contar os dias da semana
            let day_name = match task.complete_date.weekday() {
                Weekday::Mon => "Segunda-feira",
                Weekday::Tue => "Terça-feira",
                Weekday::Wed => "Quarta-feira",
                Weekday::Thu => "Quinta-feira",
                Weekday::Fri => "Sexta-feira",
                Weekday::Sat => "Sábado",
                Weekday::Sun => "Domingo",
            };
            *day_counts.entry(day_name).or_insert(0) += 1;
        }

        // 4. Encontrar o turno mais produtivo
        let most_productive_shift = shift_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(shift_name, _)| shift_name.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        // 5. Encontrar a categoria mais usada
        let most_used_category = category_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(category, _)| category)
            .unwrap_or_else(|| "N/A".to_string());

        // 6. NOVO: Encontrar o dia mais produtivo da semana
        let most_productive_day = day_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(day_name, _)| day_name.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        Ok((
            total_tasks as i64,
            executed_tasks as i64,
            percentage,
            year,
            week_num, // Alterado de `month` para `week_num`
            most_productive_shift,
            most_used_category,
            most_productive_day, // Alterado de `most_productive_week` para `most_productive_day`
        ))
    }

    pub async fn create_task(
        &self,
        task_info: &TaskDto,
        user_id: i32,
    ) -> Result<task::Model, DbErr> {
        let (begin_date, complete_date) = match task_info.r#type.as_str() {
            "MeiaHora" => (
                task_info.begin_date,
                task_info.begin_date + Duration::minutes(30),
            ),
            "UmaHora" => (
                task_info.begin_date,
                task_info.begin_date + Duration::hours(1),
            ),
            "DuasHoras" => (
                task_info.begin_date,
                task_info.begin_date + Duration::hours(1),
            ),
            "Manha" => (
                task_info.begin_date.with_hour(6).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(11).unwrap().with_minute(59).unwrap().with_second(59).unwrap(),
            ),
            "Tarde" => (
                task_info.begin_date.with_hour(12).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(17).unwrap().with_minute(59).unwrap().with_second(59).unwrap(),
            ),
            "Noite" => (
                task_info.begin_date.with_hour(18).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(23).unwrap().with_minute(59).unwrap().with_second(59).unwrap(),
            ),
            "Madrugada" => (
                task_info.begin_date.with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(5).unwrap().with_minute(59).unwrap().with_second(59).unwrap(),
            ),
            _ => {
                return Err(DbErr::Custom(format!(
                    "Invalid task type: {}",
                    task_info.r#type
                )))
            }
        };

        if begin_date < Utc::now() {
            return Err(DbErr::Custom(
                "Task begin date cannot be in the past.".to_string(),
            ));
        }

        let overlapping_task = task::Entity::find()
            .filter(task::Column::UserId.eq(user_id))
            .filter(
                Condition::all()
                    .add(task::Column::BeginDate.lt(complete_date))
                    .add(task::Column::CompleteDate.gt(begin_date)),
            )
            .one(self.db)
            .await?;

        if overlapping_task.is_some() {
            return Err(DbErr::Custom(
                "Task time overlaps with an existing task.".to_string(),
            ));
        }

        let new_task = task::ActiveModel {
            title: Set(task_info.title.clone()),
            user_id: Set(user_id),
            description: Set(Some(task_info.description.clone())),
            status: Set("Pendente".to_string()),
            begin_date: Set(begin_date),
            complete_date: Set(complete_date),
            category: Set(task_info.category.clone()),
            r#type: Set(task_info.r#type.clone()),
            ..Default::default()
        };
        new_task.insert(self.db).await
    }

    pub async fn update_task(
        &self,
        id: i32,
        task_info: &TaskDto,
        user_id: i32
    ) -> Result<task::Model, DbErr> {
        let task_to_update = self.find_by_id(id).await?
            .ok_or(DbErr::RecordNotFound(format!("Task with id {} not found", id)))?;

        if task_to_update.user_id != user_id {
            return Err(DbErr::Custom(format!("Usuário {} não está autorizado a atualizar a tarefa {}", user_id, id)));
        }

        let mut active_task = task_to_update.into_active_model();
        active_task.title = Set(task_info.title.clone());
        active_task.description = Set(Some(task_info.description.clone()));
        active_task.begin_date = Set(task_info.begin_date);
        active_task.category = Set(task_info.category.clone());
        active_task.r#type = Set(task_info.r#type.clone());

        active_task.update(self.db).await
    }

    pub async fn update_status_task(
        &self,
        id: i32,
        status: &str,
        user_id: i32,
    ) -> Result<task::Model, DbErr> {
        let task_to_update = self.find_by_id(id).await?
            .ok_or(DbErr::RecordNotFound(format!("Task with id {} not found", id)))?;

        if task_to_update.user_id != user_id {
            return Err(DbErr::Custom(format!("Usuário {} não está autorizado a atualizar a tarefa {}", user_id, id)));
        }

        let valid_status = match status {
            "Executada" | "ParcialmenteExecutada" | "Adiada" => Ok(status.to_string()),
            _ => Err(DbErr::Custom(format!("Invalid status: {}", status))),
        }?;

        let mut active_task = task_to_update.into_active_model();
        active_task.status = Set(valid_status);

        active_task.update(self.db).await
    }

    pub async fn delete_task(&self, user_id: i32, id: i32) -> Result<DeleteResult, DbErr> {
        let task_to_delete = self.find_by_id(id).await?
            .ok_or(DbErr::RecordNotFound(format!("Task with id {} not found", id)))?;

        if task_to_delete.user_id != user_id {
            return Err(DbErr::Custom(format!("Usuário {} não está autorizado a excluir a tarefa {}", user_id, id)));
        }

        task::Entity::delete_by_id(id).exec(self.db).await
    }
}