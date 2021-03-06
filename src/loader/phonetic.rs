//! Implementation for CNS code to phonetic file(s) (Zhuyin / 注音)

use super::{Loader, PathResolver};
use crate::types::{CharInfo, CnsCode};
use anyhow::Result;
use std::collections::HashMap;

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

impl PathResolver for PhoneticLoader {}

const CNS_TO_PHONETIC_FILES: [&str; 1] = ["CNS_phonetic.txt"];

/// Load and map all phonetics into an existing HashMap
pub fn load_into(load_dir: &str, map: &mut HashMap<CnsCode, CharInfo>) -> Result<()> {
  let loader = PhoneticLoader {};
  let file_paths = loader.get_files_paths(load_dir, &CNS_TO_PHONETIC_FILES);

  loader.load_into_map(map, &file_paths);

  Ok(())
}
