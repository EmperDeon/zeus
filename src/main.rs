use clap::Clap;

mod deploy;
mod dismiss;
mod setup;
mod upgrade;
mod util;

// Recommendation: Place all configurable variables to a config file
#[derive(Clap, Debug, Clone)]
#[clap(version = clap::crate_version ! (), author = clap::crate_authors ! ("\n"))]
pub struct GlobalOpts {
  /// Location for config file
  #[clap(short, long, default_value = "~/.config/monal.yaml")]
  config: String,

  /// A level of verbosity, can be used multiple times
  #[clap(short, long, parse(from_occurrences))]
  verbose: i32,

  #[clap(subcommand)]
  command: SubCommand,
}

#[derive(Clap, Debug, Clone)]
enum SubCommand {
  /// Reformat and upgrade config files, Pulls all dependant repositories
  #[clap(version = clap::crate_version ! (), author = clap::crate_authors ! ("\n"))]
  Upgrade,

  /// Setup services and their dependencies
  #[clap(version = clap::crate_version ! (), author = clap::crate_authors ! ("\n"))]
  Setup(setup::Opts),

  /// Deploy all services to Kubernetes
  #[clap(version = clap::crate_version ! (), author = clap::crate_authors ! ("\n"))]
  Deploy(deploy::Opts),

  /// Dismiss all services from Kubernetes
  #[clap(version = clap::crate_version ! (), author = clap::crate_authors ! ("\n"))]
  Dismiss(deploy::Opts)
}

fn main() {
  let opts: GlobalOpts = GlobalOpts::parse();

  util::logger::init(&opts);
  log::trace!("Arguments: {:?}", opts);

  let result = match opts.clone().command {
    SubCommand::Upgrade => { upgrade::call(opts) }
    SubCommand::Setup(sub_opts) => { setup::call(opts, sub_opts) }
    SubCommand::Deploy(sub_opts) => { deploy::call(opts, sub_opts) }
    SubCommand::Dismiss(sub_opts) => { dismiss::call(opts, sub_opts) }
  };

  std::process::exit(match result {
    Ok(code) => { code }
    Err(message) => {
      log::error!("{}", message);
      1
    }
  })
}
