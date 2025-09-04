use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StatusUpdateDto {
    #[validate(length(min = 8))]
    pub status: String,
}