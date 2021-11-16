use clap::{crate_version, Parser};
use std::error::Error;

use zh_open_data::OpenData;

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

fn main() -> Result<(), Box<dyn Error>> {
  let opts: Opts = Opts::parse();
  let chars: Vec<char> = opts.input.chars().collect();

  let data = OpenData::build(&opts.load_dir);

  for char in chars {
    if let Some(char_infos) = data.get_char_infos(char) {
      println!("{:#?}", char_infos);
    } else {
      println!("{} : Not found", char);
    }
  }

  Ok(())
}
