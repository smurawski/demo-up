use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FileDownload {
    pub file_name: Option<String>,
    pub url: Option<String>,
}

impl Default for FileDownload {
    fn default() -> Self {
        FileDownload {
            file_name: None,
            url: None,
        }
    }
}
