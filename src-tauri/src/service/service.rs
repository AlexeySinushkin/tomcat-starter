

pub enum RunState{
    Idle,
    Running
}

pub struct ServerRunState{
    pub state: RunState,
    pub run_name: String
}

pub struct Service {
    config: Configuration,
    //pub get_all_runs: Fn() -> Vec<ServerRunState>,
    //pub start_run: Fn(run_name) -> Result<(), dyn Error>
}

impl Service {
    fn new(config: Configuration) -> Service {
        Service {
            config
        }
    }
}