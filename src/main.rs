use std::env;
use std::error::Error;

use zh_open_data::get_char_infos;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();

  if args.len() >= 2 {
    let query = args[1].clone();
    let chars: Vec<char> = query.chars().collect();

    for char in chars {
      if let Some(char_infos) = get_char_infos(char) {
        println!("{:#?}", char_infos);
      } else {
        println!("Not found");
      }
    }
  }

  Ok(())
}
