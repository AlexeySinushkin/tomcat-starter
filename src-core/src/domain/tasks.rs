
pub enum TaskType {
    CopyWarRandomType(CopyWarToRandomDir),
    RunTomcatType(RunTomcat),
}

pub struct CopyWarToRandomDir {
    pub id: String,
    pub source_war_path: String,
    pub catalina_base_copy_src: String,
    pub catalina_base_copy_dst: String,
    pub destination_war_name: String
}

pub struct RunTomcat {
    pub id: String,
    pub catalina_opts: String,
    pub listen_port: String,
    pub jdpa_port: String
}

