

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthDto {
    pub username: String,
    pub password: String,
}
