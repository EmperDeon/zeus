use clap::Clap;

use crate::util;
use crate::util::config;
use crate::util::config::Config;

#[derive(Clap, Debug, Clone)]
pub struct Opts {
  /// Branch to checkout
  #[clap(short, long, default_value = "test")]
  pub(crate) environment: String,

  /// Cleanup after deploy
  #[clap(long)]
  cleanup: bool,
}

pub fn call(opts: crate::GlobalOpts, sub_opts: Opts) -> Result<i32, String> {
  let config = match config::fetch(&opts) {
    Ok(c) => { c }
    Err(str) => { return Err(str); }
  };

  for name in util::services::list(&config.root_path).unwrap() {
    if let Err(str) = deploy(&config, name.as_str(), &sub_opts) { return Err(str); }
  }

  Ok(0)
}

/// Deploys service to K8S using werf
///   Skips service if it does not have werf.yaml file (required for werf)
///
fn deploy(config: &Config, name: &str, sub_opts: &Opts) -> Result<(), String> {
  let service_path = std::path::Path::new(&config.root_path).join(name);
  let required_file = service_path.join("werf.yaml");

  if !required_file.exists() {
    log::debug!("Not found {}, skipping deploy", required_file.to_str().unwrap());
    return Ok(());
  }

  let exit_code = std::process::Command::new("werf")
    .arg("converge")
    .arg("--env")
    .arg(&sub_opts.environment)
    .arg("--repo")
    .arg(format!("{}/{}", &config.werf.repository, name))
    .current_dir(&service_path)
    .spawn().expect("werf converge failed")
    .wait().unwrap();
  println!();

  if !exit_code.success() { return Err("werf converge failed".to_owned()); }

  let exit_code = std::process::Command::new("werf")
    .arg("cleanup")
    .arg("--env")
    .arg(&sub_opts.environment)
    .arg("--repo")
    .arg(format!("{}/{}", &config.werf.repository, name))
    .current_dir(service_path)
    .spawn().expect("werf cleanup failed")
    .wait().unwrap();
  println!();

  if !exit_code.success() { return Err("werf cleanup failed".to_owned()); }

  Ok(())
}
