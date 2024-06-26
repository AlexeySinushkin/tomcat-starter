use super::property::{CommonShape, Property};
use super::tasks::TaskType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Variables {
    #[serde(rename = "globalVars")]
    pub global_vars: Vec<Property>,
    pub releases: Vec<CommonShape>,
    pub platforms: Vec<CommonShape>,
    pub servers: Vec<CommonShape>,
}

pub struct ServerRun {
    pub server_run_name: String,
    pub server_type: String, //reference to Server
    pub release: String,
    pub platform: String,
    pub tasks: Vec<TaskType>,
}

pub struct Configuration {
    pub vars: Variables,
    pub runs: Vec<ServerRun>,
}

