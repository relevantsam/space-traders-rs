use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants::limits::MAX_PLAYER_NAME_LEN;

#[derive(Deserialize, Serialize)]
pub struct Name(pub String);

impl Name {
    pub fn is_valid(&self) -> bool {
        let Name(current) = self;
        !current.is_empty() && current.len() <= MAX_PLAYER_NAME_LEN.into()
    }

    pub fn set(&mut self, name: &mut String) {
        if name.len() > MAX_PLAYER_NAME_LEN.into() {
            name.truncate(MAX_PLAYER_NAME_LEN.into());
        }
        self.0 = name.to_string();
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
