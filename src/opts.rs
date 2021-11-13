//! Tool options

use clap::{crate_version, Parser};
use lazy_static::lazy_static;

lazy_static! {
  /// Static options
  pub static ref OPTS: Opts = Opts::parse();
}

/// Options structure
#[derive(Parser)]
#[clap(version = crate_version!())]
pub struct Opts {
  /// Sets a custom load directory
  #[clap(short, long, default_value = "data")]
  pub load_dir: String,
  /// Character(s) to look informations for
  #[clap(default_value = "")]
  pub input: String,
}
