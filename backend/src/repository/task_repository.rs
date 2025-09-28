use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set};
use chrono::{Timelike, Utc, Duration};
use crate::dto::task_dto::TaskDto;
use crate::dto::task_update_dto::TaskUpdateDto;
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
        task_info: &TaskUpdateDto,
        user_id: i32
    ) -> Result<task::Model, DbErr> {
        let task_to_update = self.find_by_id(id).await?
            .ok_or(DbErr::RecordNotFound(format!("Task with id {} not found", id)))?;

        if task_to_update.user_id != user_id {
            return Err(DbErr::Custom(format!("Usuário {} não está autorizado a atualizar a tarefa {}", user_id, id)));
        }

        let status = if let Some(status) = &task_info.status {
            match status.as_str() {
                "Concluída" | "Adiada" | "Pendente" => Ok(status.clone()),
                _ => Err(DbErr::Custom(format!("Status inválido: {}", status))),
            }?
        } else {
            task_to_update.status.clone() // Manter o status existente
        };

        let mut active_task = task_to_update.into_active_model();


        if let Some(title) = &task_info.title {
            active_task.title = Set(title.clone());
        }

        match &task_info.description {
            Some(desc) => active_task.description = Set(Some(desc.clone())),
            None => {} // Manter a descrição existente
        }

        if let Some(category) = &task_info.category {
            active_task.category = Set(category.clone());
        }

        active_task.status = Set(status);
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