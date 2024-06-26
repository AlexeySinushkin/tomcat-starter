use serde::{Serialize, Deserialize};
use super::property::Property;

#[derive(Deserialize, Serialize, Debug)]
pub struct Release {
    pub name: String,    
    pub properties: Vec<Property>
}
