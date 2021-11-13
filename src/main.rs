use std::error::Error;

use zh_open_data::get_char_infos;
mod opts;

fn main() -> Result<(), Box<dyn Error>> {
  let chars: Vec<char> = opts::OPTS.input.chars().collect();

  for char in chars {
    if let Some(char_infos) = get_char_infos(char) {
      println!("{:#?}", char_infos);
    } else {
      println!("{} : Not found", char);
    }
  }

  Ok(())
}
