use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Property {
    pub name: String,    
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommonShape {
    pub name: String,    
    pub properties: Vec<Property>
}
