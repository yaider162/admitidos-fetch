use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub pun_uptc: String,
    pub factors: Vec<String>,
    pub admitido: bool,
    pub program: String,
}