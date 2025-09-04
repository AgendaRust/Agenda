use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set};
use crate::dto::taskDTO::TaskDto;
use crate::entity::task;
use chrono::{Duration, Timelike, Utc};

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
            "Manha" => (
                task_info.begin_date.with_hour(8).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(12).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
            ),
            "Tarde" => (
                task_info.begin_date.with_hour(13).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(18).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
            ),
            "Noite" => (
                task_info.begin_date.with_hour(18).unwrap().with_minute(0).unwrap().with_second(0).unwrap(),
                task_info.begin_date.with_hour(23).unwrap().with_minute(59).unwrap().with_second(59).unwrap(),
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
            description: Set(task_info.description.clone()),
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
    ) -> Result<task::Model, DbErr> {
        let task_to_update = self.find_by_id(id).await?
            .ok_or(DbErr::RecordNotFound(format!("Task with id {} not found", id)))?;

        let mut active_task = task_to_update.into_active_model();
        active_task.title = Set(task_info.title.clone());
        active_task.description = Set(task_info.description.clone());
        active_task.begin_date = Set(task_info.begin_date);
        active_task.category = Set(task_info.category.clone());
        active_task.r#type = Set(task_info.r#type.clone());

        active_task.update(self.db).await
    }

    pub async fn update_status_task(
        &self,
        id: i32,
        status: &str,
    ) -> Result<task::Model, DbErr> {
        let task_to_update = self.find_by_id(id).await?
            .ok_or(DbErr::RecordNotFound(format!("Task with id {} not found", id)))?;

        let valid_status = match status {
            "Executada" | "ParcialmenteExecutada" | "Adiada" => Ok(status.to_string()),
            _ => Err(DbErr::Custom(format!("Invalid status: {}", status))),
        }?;

        let mut active_task = task_to_update.into_active_model();
        active_task.status = Set(valid_status);

        active_task.update(self.db).await
    }

    pub async fn delete_task(&self, id: i32) -> Result<DeleteResult, DbErr> {
        task::Entity::delete_by_id(id).exec(self.db).await
    }
}