pub mod user1_rest;
pub mod user_rest;
pub mod redis_info_rest;

#[derive(strum_macros::Display)]
pub enum RespError {
    UNKNOWN
}