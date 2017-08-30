use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct Item {
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    pub icon: Icon,
}

#[derive(Debug, Serialize)]
pub struct Icon {
    pub path: PathBuf,
}
