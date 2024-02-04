use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Release {
    pub name: String,    
    pub properties: Vec<Property>
}
