use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    pub name: String,    
    pub properties: Vec<Property>
}
