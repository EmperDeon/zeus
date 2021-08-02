use crate::util;
use crate::util::config;
use crate::util::config::Config;

pub fn call(opts: crate::GlobalOpts, sub_opts: crate::deploy::Opts) -> Result<i32, String> {
  let config = match config::fetch(&opts) {
    Ok(c) => { c }
    Err(str) => { return Err(str); }
  };

  for name in util::services::list(&config.root_path).unwrap() {
    if let Err(str) = dismiss(&config, name.as_str(), &sub_opts) { return Err(str); }
  }

  Ok(0)
}

/// Dismisses (removes) service from K8S using werf
///   Skips service if it does not have werf.yaml file (required for werf)
///
fn dismiss(config: &Config, name: &str, sub_opts: &crate::deploy::Opts) -> Result<(), String> {
  let service_path = std::path::Path::new(&config.root_path).join(name);
  let required_file = service_path.join("werf.yaml");

  if !required_file.exists() {
    log::debug!("Not found {}, skipping dismiss", required_file.to_str().unwrap());
    return Ok(());
  }

  std::process::Command::new("werf")
    .arg("dismiss")
    .arg("--env")
    .arg(&sub_opts.environment)
    .arg("--repo")
    .arg(format!("{}/{}", &config.werf.repository, name))
    .current_dir(service_path)
    .spawn().expect("werf dismiss failed")
    .wait().unwrap();

  println!();
  Ok(())
}
