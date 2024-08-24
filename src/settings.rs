use serde::{Deserialize, Serialize};

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
