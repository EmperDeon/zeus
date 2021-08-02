use crate::util;
use std::fs::File;
use std::collections::HashMap;

pub fn list(root_path: &String) -> Result<Vec<String>, String> {
  let mut names = vec![util::git::MAIN_REPO_NAME.to_owned(), "tests".to_owned()];
  names.append(&mut config_list(root_path).unwrap());

  Ok(names)
}

fn config_list(root_path: &String) -> Result<Vec<String>, String> {
  let path = std::path::Path::new(root_path).join(util::git::MAIN_REPO_NAME).join("services.yml");

  if !path.exists() {
    if let Err(str) = util::git::clone(&root_path, util::git::MAIN_REPO_NAME) { return Err(str); }
  }

  let input = File::open(path).expect("Failed to open config file");
  let config: HashMap<String, String> = serde_yaml::from_reader(input).unwrap();

  Ok(config.keys().cloned().collect())
}
