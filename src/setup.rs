use clap::Clap;

use crate::util;
use crate::util::config;

#[derive(Clap, Debug, Clone)]
pub struct Opts {
  /// Branch to checkout
  #[clap(short, long, default_value = "master")]
  branch: String
}

pub fn call(opts: crate::GlobalOpts, sub_opts: Opts) -> Result<i32, String> {
  let config = match config::fetch(&opts) {
    Ok(c) => { c }
    Err(str) => { return Err(str); }
  };

  for name in util::services::list(&config.root_path).unwrap() {
    if let Err(str) = util::git::checkout(&config.root_path, name.as_str(), &sub_opts.branch) { return Err(str); }
  }

  Ok(0)
}
