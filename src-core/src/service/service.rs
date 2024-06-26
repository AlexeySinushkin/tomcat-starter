use crate::domain::config::Configuration;


pub enum RunState{
    Idle,
    Running{run_name: String}
}


pub struct service {
    config: Configuration,
    //pub get_all_runs: Fn() -> Vec<ServerRunState>,
    //pub start_run: Fn(run_name) -> Result<(), dyn Error>
}



impl service {
    fn new(config: Configuration) -> service {
        service {
            config
        }
    }
}

