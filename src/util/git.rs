use git2::{Repository};

pub const MAIN_REPO_NAME: &str = "main";
const GIT_REPO_BASE: &str = "ssh://git@git.jetbrains.space/monal/fmw/";

pub fn clone(root_path: &String, name: &str) -> Result<(), String> {
  log::trace!("Cloning git repository {} to {}", name, root_path);

  let path = std::path::Path::new(&root_path).join(name);
  if path.exists() { return Err(format!("Path already exists, skipping '{}'", path.to_str().unwrap())); }

  std::process::Command::new("git")
    .arg("clone")
    .arg(format!("{}/{}.git", GIT_REPO_BASE, name))
    .current_dir(root_path)
    .spawn()
    .expect("git clone failed")
    .wait().unwrap();

  println!();
  Ok(())
}

pub fn pull(root_path: &String, name: &str) -> Result<(), String> {
  let path = std::path::Path::new(root_path).join(name);
  log::trace!("Pulling git repository at {}", path.to_str().unwrap());

  if !path.exists() { if let Err(str) = clone(&root_path, name) { return Err(str); } }

  std::process::Command::new("git")
    .arg("pull")
    .current_dir(path)
    .spawn().expect("git pull failed")
    .wait().unwrap();

  println!();
  Ok(())
}

pub fn checkout(root_path: &String, name: &str, branch: &String) -> Result<(), String> {
  let path = std::path::Path::new(root_path).join(name);
  log::trace!("Checking out branch {} in git repository at {}", branch, path.to_str().unwrap());

  if !path.exists() { if let Err(str) = clone(&root_path, name) { return Err(str); } }

  std::process::Command::new("git")
    .arg("fetch")
    .arg("--all")
    .current_dir(&path)
    .spawn().expect("git fetch failed")
    .wait().unwrap();

  let repo = Repository::open(&path).unwrap();
  let has_branch = repo.branches(None).unwrap().any(|item| item.unwrap().0.name().unwrap().unwrap() == branch);

  let branch = if has_branch { branch.clone() } else {
    log::trace!("Could not find branch {} in repo, checking out master", branch);
    "master".to_owned()
  };

  let (object, reference) = repo.revparse_ext(branch.as_str()).expect("Object not found");
  repo.checkout_tree(&object, None).expect("Failed to checkout");

  match reference {
    // gref is an actual reference like branches or tags
    Some(gref) => repo.set_head(gref.name().unwrap()),
    // this is a commit, not a reference
    None => repo.set_head_detached(object.id()),
  }.expect("Failed to set HEAD");

  println!();
  Ok(())
}
