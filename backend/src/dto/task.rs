
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskDto {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
