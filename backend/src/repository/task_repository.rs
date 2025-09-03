use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set};
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

    pub async fn create_task(
        &self,
        task_info: &TaskDto,
        user_id: i32,
    ) -> Result<task::Model, DbErr> {
        let new_task = task::ActiveModel {
            title: Set(task_info.title.clone()),
            user_id: Set(user_id),
            description: Set(task_info.description.clone()),
            status: Set(task_info.status.clone()),
            begin_date: Set(task_info.begin_date),
            complete_date: Set(task_info.complete_date),
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
        active_task.status = Set(task_info.status.clone());
        active_task.begin_date = Set(task_info.begin_date);
        active_task.complete_date = Set(task_info.complete_date);
        active_task.category = Set(task_info.category.clone());
        active_task.r#type = Set(task_info.r#type.clone());

        active_task.update(self.db).await
    }

    pub async fn delete_task(&self, id: i32) -> Result<DeleteResult, DbErr> {
        task::Entity::delete_by_id(id).exec(self.db).await
    }
}