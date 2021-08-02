use crate::util;
use serde_derive::{Serialize, Deserialize};
use path_absolutize::Absolutize;
use std::fs::File;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
  pub(crate) root_path: String,

  #[serde(default)]
  pub(crate) werf: WerfConfig
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WerfConfig {
  pub(crate) repository: String
}

const CONFIG_TEMPLATE: &str = include_str!("../templates/config.yaml.mustache");

pub fn fetch(opts: &crate::GlobalOpts) -> Result<Config, String> {
  let config_path = path_prepare(&opts.config);

  if !present(&config_path) {
    log::warn!("Could not read config at {}", config_path);

    if util::ask("Create default config ?") {
      create(&config_path);
    };
  }

  read_and_validate(&config_path)
}

pub fn upgrade(opts: &crate::GlobalOpts) -> Result<Config, String> {
  let config_path = path_prepare(&opts.config);

  if !present(&config_path) { return Err(format!("No config file to upgrade {}", config_path)); }

  let config = match read_and_validate(&config_path) {
    Ok(c) => { c }
    Err(str) => { return Err(str); }
  };

  let data = mustache::MapBuilder::new()
    .insert_str("root_path", config.root_path)
    .insert_str("werf_repository", config.werf.repository)
    .build();
  let content = mustache::compile_str(CONFIG_TEMPLATE).expect("Failed to compile template").render_data_to_string(&data).expect("Failed to render config");

  log::trace!("Writing config to file: {}", config_path);

  let mut output = File::create(config_path).expect("Failed to create config file");
  write!(output, "{}", content).expect("Failed to write config");

  fetch(opts)
}

////
// Internal
////

fn path_prepare(path: &String) -> String {
  let new_path = if path.contains("~/.config/") {
    dirs::config_dir().expect("Failed to construct config dir").join(path.replace("~/.config/", ""))
  } else {
    std::path::Path::new(path).to_path_buf()
  };

  new_path.absolutize().unwrap().to_str().unwrap().to_owned()
}

fn present(config_path: &String) -> bool {
  let exists = std::path::Path::new(config_path).exists();

  if exists {
    let mut buf: Vec<u8> = Vec::new();
    let content = File::open(config_path).unwrap().read_to_end(&mut buf).unwrap();

    content > 0
  } else {
    exists
  }
}

fn create(config_path: &String) {
  let data = mustache::MapBuilder::new()
    .insert_str("root_path", "/home/example/monal")
    .insert_str("werf_repository", "registry.gitlab.com/example/images")
    .build();
  let content = mustache::compile_str(CONFIG_TEMPLATE).expect("Failed to compile template").render_data_to_string(&data).expect("Failed to render config");

  log::trace!("Writing config to file: {}", config_path);

  let mut output = File::create(config_path).expect("Failed to create config file");
  write!(output, "{}", content).expect("Failed to write config");
}

fn read_and_validate(path: &String) -> Result<Config, String> {
  let input = File::open(path).expect("Failed to open config file");
  let config: Config = serde_yaml::from_reader(input).unwrap();

  // 1. Root folder should be present and writable
  let path = std::path::Path::new(&config.root_path);
  if !path.exists() || path.metadata().unwrap().permissions().readonly() {
    return Err(format!("Root folder '{}' does not exist or is not writable", config.root_path).to_owned());
  }

  log::trace!("Read config: {:?}", config);
  Ok(config)
}
