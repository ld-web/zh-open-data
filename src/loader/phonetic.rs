//! Implementation for CNS code to phonetic file(s) (Zhuyin / 注音)

use std::collections::HashMap;
use std::io;

use crate::types::{CharInfo, CnsCode};

use super::Loader;

const CNS_TO_PHONETIC_FILES: [&str; 1] = ["data/CNS_phonetic.txt"];

struct PhoneticLoader {}

impl PhoneticLoader {
  /// Parse a given line a return a tuple containing extracted informations
  fn parse_line(line: String) -> Option<(CnsCode, String)> {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() != 2 {
      return None;
    }
    let cns_code = s[0].to_string();
    let phonetic = s[1].to_string();

    Some((cns_code, phonetic))
  }
}

impl Loader<CnsCode, CharInfo> for PhoneticLoader {
  fn process_line(map: &mut HashMap<CnsCode, CharInfo>, line: String) {
    let (cns_code, phonetic) = Self::parse_line(line).unwrap();

    if let Some(char_info) = map.get_mut(&cns_code) {
      char_info.phonetic.push(phonetic);
    }
  }
}

/// Load and map all phonetics into an existing HashMap
pub fn load_into(map: &mut HashMap<CnsCode, CharInfo>) -> Result<(), io::Error> {
  let loader = PhoneticLoader {};

  loader.load_into_map(map, &CNS_TO_PHONETIC_FILES);

  Ok(())
}
