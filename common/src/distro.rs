use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize,Clone, PartialEq)]
pub struct Distro {
    pub id: i32,
    pub url: String,
    pub name_or_path: String,
    pub key: Option<String>,
    pub armored_key: bool,
    pub flat_repo: bool,
}
