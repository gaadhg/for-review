mod repository;

pub use repository::*;
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: SmolStr,
    pub password: SmolStr,
    // pub next: Option<String>,
}
