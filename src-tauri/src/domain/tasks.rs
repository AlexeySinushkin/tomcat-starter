use serde::{Serialize, Deserialize};

pub enum TaskType {
    CopyWarRandom,
    RunTomcat,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CopyWarToRandomDir {
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub id: String,
    #[serde(rename = "sourceWarPath")]
    pub source_war_path: String,
    #[serde(rename = "destinationCatalinaBase")]
    pub destination_catalina_base: String,
    #[serde(rename = "destinationWarName")]
    pub destination_war_name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunTomcat {
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub id: String,
    #[serde(rename = "catalinaOpts")]
    pub catalina_opts: String,
    #[serde(rename = "listenPort")]
    pub listen_port: String,
    #[serde(rename = "jdpaPort")]
    pub jdpa_port: String
}