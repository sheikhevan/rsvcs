use yaml_rust2::{YamlEmitter, YamlLoader};

pub fn parse_yaml(path: &str) {
    let config_to_parse = YamlLoader::load_from_str(path).unwrap();
    let config = &config_to_parse[0];
    let backupDir = config["backupDir"][0].as_str();
    println!("{:?}", backupDir);
}
