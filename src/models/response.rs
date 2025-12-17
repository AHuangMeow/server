use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AboutMe {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub username: String,
    pub is_admin: bool,
}
