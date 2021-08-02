use crate::util;
use crate::util::config;

pub fn call(opts: crate::GlobalOpts) -> Result<i32, String> {
  let config = match config::fetch(&opts) {
    Ok(c) => { c }
    Err(str) => { return Err(str); }
  };

  if let Err(str) = config::upgrade(&opts) { return Err(str); }

  for name in util::services::list(&config.root_path).unwrap() {
    if let Err(str) = util::git::pull(&config.root_path, name.as_str()) { return Err(str); }
  }

  Ok(0)
}
