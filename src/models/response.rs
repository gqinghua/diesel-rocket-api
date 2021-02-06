use serde_json::Value;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response {
//     pub message: String,
//     pub data: Value,
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct Response{
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}