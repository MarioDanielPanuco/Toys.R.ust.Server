use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub(crate) id: i64,
    pub(crate) health: i64,
    pub(crate) deadeye: i64,
}
