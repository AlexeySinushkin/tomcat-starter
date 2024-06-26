use std::path::Path;
use ts_core::domain::tasks::CopyWarToRandomDir;
use ts_core::service::task_copy_random;

#[test]
pub fn copy_war() {
    let config = CopyWarToRandomDir {
        id: "1".to_string(),
        source_war_path: "./tests/resources/dedo_v1.war".to_string(),
        catalina_base_copy_src: "./tests/resources/tomcat".to_string(),
        catalina_base_copy_dst: "./target/tmp".to_string(),
        destination_war_name: "dedo.war".to_string(),
    };
    println!("source war path {:?}", Path::new(&config.source_war_path).canonicalize().unwrap());
    let result = task_copy_random::perform(&config);
    let exists = result.unwrap()
        .join("webapps")
        .join("dedo.war")
        .exists();
    assert!(exists);
}

