use crate::map;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AutoIncrVars(pub HashMap<String, u32>);

impl AutoIncrVars {
    pub fn default() -> AutoIncrVars {
        Self(map![
            "log_id".to_string() => 0,
            "vault_id".to_string() => 0,
            "archive_id".to_string() => 0,
        ])
    }
}
