//! Implementation for CNS code to unicode file(s)

use super::{Loader, PathResolver};
use crate::{
  types::{CharInfo, CnsCode, UnicodeHexVal},
  utils,
};
use anyhow::Result;
use std::collections::HashMap;

struct UnicodeToCnsLoader {}

impl UnicodeToCnsLoader {
  /// Parse a given line a return a tuple containing extracted informations
  fn parse_line(line: String) -> Option<(UnicodeHexVal, CnsCode)> {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() != 2 {
      return None;
    }
    let cns_code = s[0].to_string();
    let unicode = s[1].to_string();

    Some((unicode, cns_code))
  }
}

impl Loader<UnicodeHexVal, CnsCode> for UnicodeToCnsLoader {
  /// For a HashMap<UnicodeHexVal, CnsCode>, just put the Key/Value into the map.
  fn process_line(map: &mut HashMap<UnicodeHexVal, CnsCode>, line: String) {
    let (unicode, cns_code) = Self::parse_line(line).unwrap();
    map.insert(unicode, cns_code);
  }
}

impl Loader<CnsCode, CharInfo> for UnicodeToCnsLoader {
  /// For HashMap<CnsCode, CharInfo>, build a default CharInfo with found unicode hex value & CNS Code
  ///
  /// The CharInfo is then put into the map, and other Loader will be able to put their own informations into it
  fn process_line(map: &mut HashMap<CnsCode, CharInfo>, line: String) {
    let (unicode, cns_code) = Self::parse_line(line).unwrap();

    let char_info = CharInfo::new(utils::get_char(unicode), cns_code.clone());

    map.insert(cns_code, char_info);
  }
}

impl PathResolver for UnicodeToCnsLoader {}

const CNS_TO_UNICODE_FILES: [&str; 3] = [
  "CNS2UNICODE_Unicode_2.txt",
  "CNS2UNICODE_Unicode_BMP.txt",
  "CNS2UNICODE_Unicode_15.txt",
];

/// This function will give a HashMap with unicode hex value as keys and CNS Code as values
pub fn get_single_map(load_dir: &str) -> Result<HashMap<UnicodeHexVal, CnsCode>> {
  let loader = UnicodeToCnsLoader {};
  let file_paths = loader.get_files_paths(load_dir, &CNS_TO_UNICODE_FILES);
  let data = loader.get_map(&file_paths)?;
  Ok(data)
}

/// Load and map all unicode values into an existing HashMap
pub fn load_into(load_dir: &str, map: &mut HashMap<CnsCode, CharInfo>) -> Result<()> {
  let loader = UnicodeToCnsLoader {};
  let file_paths = loader.get_files_paths(load_dir, &CNS_TO_UNICODE_FILES);

  loader.load_into_map(map, &file_paths);

  Ok(())
}
