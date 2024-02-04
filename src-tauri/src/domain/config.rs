use serde::{Deserialize, Serialize};
use super::property::Property;
use super::tasks::{CopyWarToRandomDir,  RunTomcat, TaskType};


#[derive(Deserialize, Serialize, Debug)]
pub struct Variables {
    #[serde(rename = "globalVars")]
    pub global_vars: Vec<Property>,
    pub releases: Vec<Property>,
    pub platforms: Vec<Property>,
    pub servers: Vec<Property>,
}

pub struct ServerRun {
    pub server_run_name: String,
    pub server_name: String,    //reference to Server
    pub tasks: Vec<TaskType>,
}

pub struct Configuration {
    pub vars: Variables,
    pub runs: Vec<ServerRun>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigurationDto {
    pub vars: Variables,
    pub runs: Vec<ServerRunDto>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerRunDto {
    #[serde(rename = "serverRunName")]
    pub server_run_name: String,
    
    #[serde(rename = "serverName")]
    pub server_name: String,
    
    pub order:Vec<String>,

    #[serde(rename = "warToRandom")]
    pub war_to_random: Vec<CopyWarToRandomDirDto>,

    #[serde(rename = "runTomcat")]
    pub run_tomcat: Vec<RunTomcatDto>,
}



#[derive(Deserialize, Serialize, Debug)]
pub enum TaskTypeDto {
    CopyWarRandomType,
    RunTomcatType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CopyWarToRandomDirDto {
    #[serde(rename = "type")]
    pub task_type: TaskTypeDto,
    pub id: String,
    #[serde(rename = "sourceWarPath")]
    pub source_war_path: String,
    #[serde(rename = "destinationCatalinaBase")]
    pub destination_catalina_base: String,
    #[serde(rename = "destinationWarName")]
    pub destination_war_name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunTomcatDto {
    #[serde(rename = "type")]
    pub task_type: TaskTypeDto,
    pub id: String,
    #[serde(rename = "catalinaOpts")]
    pub catalina_opts: String,
    #[serde(rename = "listenPort")]
    pub listen_port: String,
    #[serde(rename = "jdpaPort")]
    pub jdpa_port: String
}


pub fn convertToDto(config: Configuration) {
   /* let runs : Vec<ServerRunDto> = vec![];
    for item in config.runs.iter() {

    }
    */
}