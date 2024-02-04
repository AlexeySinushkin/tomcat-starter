use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Platform {
    pub name: String,    
    pub properties: Vec<Property>
}
