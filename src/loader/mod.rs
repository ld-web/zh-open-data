//! Generic trait for file loading

use std::collections::HashMap;
use std::io;
use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub mod char_info;
pub mod components;
pub mod phonetic;
pub mod stroke;
pub mod unicode_to_cns;

trait Loader<K, V> {
  /// Load and map given paths into an existing map
  fn load_into_map(&self, map: &mut HashMap<K, V>, paths: &[String]) {
    for path in paths.iter() {
      Self::load_into(map, path).unwrap();
    }
  }

  /// Get a single map with given paths loaded and mapped into it
  fn get_map(&self, paths: &[String]) -> Result<HashMap<K, V>, io::Error> {
    let mut data: HashMap<K, V> = HashMap::new();

    for path in paths.iter() {
      Self::load_into(&mut data, path)?;
    }

    Ok(data)
  }

  /// Load a given file into a given map
  fn load_into(map: &mut HashMap<K, V>, path: &str) -> Result<(), io::Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
      let content = line.unwrap();
      Self::process_line(map, content);
    }

    Ok(())
  }

  /// Given a map with concrete types, and a line, do something : process the line and put the result into the map
  fn process_line(map: &mut HashMap<K, V>, line: String);
}

trait PathResolver {
  /// Given an array of filenames (only filenames, not full path), resolve the full path based on provided CLI options
  ///
  /// Essentially, it will build all the paths by concatenating the `load-dir` option with the filename
  fn get_files_paths(&self, load_dir: &str, filenames: &[&str]) -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();

    for filename in filenames.iter() {
      let path = format!("{}/{}", load_dir, filename);
      file_paths.push(path);
    }

    file_paths
  }
}
