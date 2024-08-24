use serde::{Deserialize, Serialize};

use crate::cli::Args;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Settings {
    /// Maximum width of each line
    pub max_width: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self { max_width: 100 }
    }
}

impl Settings {
    fn new() -> Self {
        Settings::default()
    }
}
