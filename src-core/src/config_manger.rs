
use super::domain::tasks::{CopyWarToRandomDir, RunTomcat, TaskType};
use crate::domain::config::{Configuration, ServerRun, Variables};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::BTreeMap;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const CONFIG_FILE_NAME: &str = "config.json";

pub struct ConfigManager {
    pub current: Option<ConfigurationDto>,
}

impl ConfigManager {
    pub fn set(&mut self, config: ConfigurationDto) {
        self.borrow_mut().current = Some(config);
        self.save();
    }
    pub fn get(&mut self) -> Option<&ConfigurationDto> {
        if self.current.is_none() {
            self.load();            
        }
        return self.borrow_mut().current.as_ref();   
    }
    fn save(&mut self) {
        //-> Result<(), Box<dyn Error>>
        let mut output = File::create(Path::new(&CONFIG_FILE_NAME)).expect("Unable to create file");
        if let Some(config) = &self.current {
            let j = serde_json::to_string(&config).expect("Serialize dto to json");
            output.write(j.as_bytes()).expect("Write config file");
        }
    }
    fn load(&mut self) {
        let input = File::open(Path::new(&CONFIG_FILE_NAME));
        if let Ok(mut f) = input {
            let mut buffer = String::new();
            if let Ok(_) = f.read_to_string(&mut buffer) {
                let config_result = serde_json::from_str::<ConfigurationDto>(&buffer);
                if let Ok(dto) = config_result {
                    self.borrow_mut().current = Some(dto);
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigurationDto {
    pub vars: Variables,
    pub runs: Vec<ServerRunDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerRunDto {
    #[serde(rename = "serverRunName")]
    pub server_run_name: String,

    #[serde(rename = "serverType")]
    pub server_type: String,

    pub order: Vec<String>,

    #[serde(rename = "warToRandom")]
    pub war_to_random: Vec<CopyWarToRandomDirDto>,

    #[serde(rename = "runTomcat")]
    pub run_tomcat: Vec<RunTomcatDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TaskTypeDto {
    CopyWarRandomType,
    RunTomcatType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CopyWarToRandomDirDto {
    pub id: String,
    #[serde(rename = "sourceWarPath")]
    pub source_war_path: String,
    #[serde(rename = "sourceCatalinaBase")]
    pub source_catalina_base: String,
    #[serde(rename = "destinationCatalinaBase")]
    pub destination_catalina_base: String,
    #[serde(rename = "destinationWarName")]
    pub destination_war_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
            server_type: run.server_type,
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
            server_type: run.server_type,
            release: "".to_string(),
            server_run_name: run.server_run_name,
            tasks: tasksOrdered,
            platform: "".to_string(),
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
        destination_catalina_base: task.catalina_base_copy_dst,
        source_catalina_base: task.catalina_base_copy_src,
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
        catalina_base_copy_dst: task.destination_catalina_base,
        catalina_base_copy_src: task.source_catalina_base,
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
