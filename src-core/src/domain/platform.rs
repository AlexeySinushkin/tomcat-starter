use serde::{Serialize, Deserialize};
use super::property::Property;

#[derive(Deserialize, Serialize, Debug)]
pub struct Platform {
    pub name: String,    
    pub properties: Vec<Property>
}
