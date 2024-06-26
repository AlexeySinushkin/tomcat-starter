use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use crate::domain::tasks::CopyWarToRandomDir;
use uuid::Uuid;

const WEBAPPS: &str = "webapps";

pub fn perform(config: &CopyWarToRandomDir) -> Result<PathBuf, Box<dyn Error>>{
    let src_war_path = Path::new(&config.source_war_path).to_path_buf();

    let id = format!("{}", Uuid::new_v4().as_simple());
    let dst_war_path = Path::new(&config.catalina_base_copy_dst)
        .to_path_buf()
        .join(id)
        .join(WEBAPPS);

    fs::create_dir_all(dst_war_path.clone())?;

    let dst_war_path= dst_war_path.join(&config.destination_war_name);
    fs::copy(src_war_path, dst_war_path.clone())?;

    Ok(dst_war_path)
}