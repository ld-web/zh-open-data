//! Generic trait for file loading

use std::collections::HashMap;
use std::io;
use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub mod components;
pub mod phonetic;
pub mod stroke;
pub mod unicode_to_cns;

trait Loader<K, V> {
  fn load_files_to_map(&self, paths: &[&str]) -> Result<HashMap<K, V>, io::Error> {
    let mut data: HashMap<K, V> = HashMap::new();

    for path in paths.iter() {
      Self::load_into(&mut data, path)?;
    }

    Ok(data)
  }

  fn load_into(map: &mut HashMap<K, V>, path: &str) -> Result<(), io::Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
      let content = line.unwrap();
      Self::process_line(map, content);
    }

    Ok(())
  }

  fn process_line(map: &mut HashMap<K, V>, line: String);
}
