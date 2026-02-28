use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LucideIcon {
    pub categories: Vec<String>,
}
