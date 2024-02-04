use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    pub name: String,    
    pub value: String,
}
