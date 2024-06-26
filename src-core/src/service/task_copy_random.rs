use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use copy_dir::copy_dir;
use crate::domain::tasks::CopyWarToRandomDir;
use uuid::Uuid;

const WEBAPPS: &str = "webapps";

type Err = Box<dyn Error>;

pub fn perform(config: &CopyWarToRandomDir) -> Result<PathBuf, Err> {
    let id = format!("{}", Uuid::new_v4().as_simple());
    let catalina_base = Path::new(&config.catalina_base_copy_dst)
        .to_path_buf()
        .join(id);

    copy_war(config, &catalina_base)?;
    copy_conf(config, &catalina_base)?;

    Ok(catalina_base)
}

fn copy_war(config: &CopyWarToRandomDir, catalina_base: &PathBuf) -> Result<(), Err> {
    let src_war_path = Path::new(&config.source_war_path).to_path_buf();
    let dst_war_path = catalina_base
        .clone()
        .join(WEBAPPS);

    fs::create_dir_all(dst_war_path.clone())?;

    let dst_war_path = dst_war_path
        .join(&config.destination_war_name);
    fs::copy(src_war_path, dst_war_path)?;
    Ok(())
}

fn copy_conf(config: &CopyWarToRandomDir, catalina_base: &PathBuf) -> Result<(), Err> {
    let src_conf_path = Path::new(&config.catalina_base_copy_src)
        .to_path_buf()
        .join("conf");
    let dst_conf_path = catalina_base
        .clone()
        .join("conf");

    copy_dir(src_conf_path, dst_conf_path)?;
    Ok(())
}
