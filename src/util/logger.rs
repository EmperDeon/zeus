use crate::GlobalOpts;
use simplelog::{LevelFilter, ConfigBuilder, LevelPadding, TerminalMode, ColorChoice};

pub fn init(opts: &GlobalOpts) {
  let level: LevelFilter = match opts.verbose {
    0 => LevelFilter::Info,
    1 => LevelFilter::Debug,
    2 | _ => LevelFilter::Trace
  };

  let mut config = ConfigBuilder::new();
  config.set_thread_level(LevelFilter::Off);
  config.set_location_level(LevelFilter::Off);
  config.set_target_level(LevelFilter::Off);
  config.set_level_padding(LevelPadding::Left);
  config.set_time_to_local(false);

  // Set offset to current time to get initial time to be 00:00:00
  let time = chrono::Utc::now().timestamp() - chrono::Utc::today().and_hms(0, 0, 0).timestamp();
  let offset = chrono::FixedOffset::east(-time as i32);
  config.set_time_offset(offset);

  simplelog::TermLogger::init(level, config.build(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

  log::debug!("Initialized with debug level: {}", level);
}
