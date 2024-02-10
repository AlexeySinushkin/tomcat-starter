use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};

use super::property::Property;
use super::tasks::{CopyWarToRandomDir, RunTomcat, TaskType};
use serde::{Deserialize, Serialize};

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
    pub server_name: String, //reference to Server
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

    pub order: Vec<String>,

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
    pub id: String,
    #[serde(rename = "sourceWarPath")]
    pub source_war_path: String,
    #[serde(rename = "destinationCatalinaBase")]
    pub destination_catalina_base: String,
    #[serde(rename = "destinationWarName")]
    pub destination_war_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunTomcatDto {
    pub id: String,
    #[serde(rename = "catalinaOpts")]
    pub catalina_opts: String,
    #[serde(rename = "listenPort")]
    pub listen_port: String,
    #[serde(rename = "jdpaPort")]
    pub jdpa_port: String,
}

pub fn convert_to_dto(config: Configuration) -> ConfigurationDto {
    let mut runs: Vec<ServerRunDto> = vec![];

    for run in config.runs.into_iter() {
        let mut war_to_random: Vec<CopyWarToRandomDirDto> = vec![];
        let mut run_tomcat: Vec<RunTomcatDto> = vec![];
        let mut order: Vec<String> = vec![];
        for task in run.tasks.into_iter() {
            if let TaskType::CopyWarRandomType(task) = task {
                order.push(task.id.clone());
                war_to_random.push(map_copy_war_to_dto(task));
            } else if let TaskType::RunTomcatType(task) = task {
                order.push(task.id.clone());
                run_tomcat.push(map_run_tomcat_to_dto(task));
            }
        }
        runs.push(ServerRunDto {
            server_name: run.server_name,
            server_run_name: run.server_run_name,
            war_to_random,
            run_tomcat,
            order,
        })
    }
    return ConfigurationDto {
        vars: config.vars,
        runs,
    };
}

pub fn convert_from_dto(config: ConfigurationDto) -> Configuration {
    let mut runs: Vec<ServerRun> = vec![];
    for run in config.runs.into_iter() {
        //let mut tasks : Vec<TaskType> = vec![];
        let mut tasksMap = BTreeMap::new();

        for task in run.war_to_random.into_iter() {
            let id = task.id.clone();
            let task = TaskType::CopyWarRandomType(map_copy_war_from_dto(task));
            tasksMap.insert(id, task);
        }
        for task in run.run_tomcat.into_iter() {
            let id = task.id.clone();
            let task = TaskType::RunTomcatType(map_run_tomcat_from_dto(task));
            tasksMap.insert(id, task);
        }
        let mut tasksOrdered: Vec<TaskType> = vec![];
        for taskId in run.order.iter() {
            let task = tasksMap.remove(taskId).unwrap();
            tasksOrdered.push(task);
        }
        runs.push(ServerRun {
            server_name: run.server_name,
            server_run_name: run.server_run_name,
            tasks: tasksOrdered,
        })
    }
    return Configuration {
        vars: config.vars,
        runs,
    };
}

fn map_copy_war_to_dto(task: CopyWarToRandomDir) -> CopyWarToRandomDirDto {
    CopyWarToRandomDirDto {
        id: task.id,
        destination_catalina_base: task.destination_catalina_base,
        destination_war_name: task.destination_war_name,
        source_war_path: task.source_war_path,
    }
}

fn map_run_tomcat_to_dto(task: RunTomcat) -> RunTomcatDto {
    RunTomcatDto {
        id: task.id,
        catalina_opts: task.catalina_opts,
        listen_port: task.listen_port,
        jdpa_port: task.jdpa_port,
    }
}

fn map_copy_war_from_dto(task: CopyWarToRandomDirDto) -> CopyWarToRandomDir {
    CopyWarToRandomDir {
        id: task.id,
        destination_catalina_base: task.destination_catalina_base,
        destination_war_name: task.destination_war_name,
        source_war_path: task.source_war_path,
    }
}

fn map_run_tomcat_from_dto(task: RunTomcatDto) -> RunTomcat {
    RunTomcat {
        id: task.id,
        catalina_opts: task.catalina_opts,
        listen_port: task.listen_port,
        jdpa_port: task.jdpa_port,
    }
}
