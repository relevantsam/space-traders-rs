use serde;
use serde::{Deserialize, Serialize};

use super::user_name::Name;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct UserState {
    pub name: Name,
    pub token: Option<String>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            name: Name(String::new()),
            token: None,
        }
    }
}
