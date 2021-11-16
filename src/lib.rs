//! API entry for Chinese characters Open Data

use std::collections::HashMap;
use types::{CharInfo, CnsCode, UnicodeHexVal};

mod loader;
pub mod types;
mod utils;

pub struct OpenData {
  unicode_to_cns: HashMap<UnicodeHexVal, CnsCode>,
  global_map: HashMap<CnsCode, CharInfo>,
}

impl OpenData {
  /// Build all the hashmaps based on provided loading directory
  pub fn build(load_dir: &str) -> OpenData {
    OpenData {
      unicode_to_cns: loader::unicode_to_cns::get_single_map(load_dir).unwrap(),
      global_map: loader::char_info::get_global_map(load_dir).unwrap(),
    }
  }

  /// Get a single character infos
  pub fn get_char_infos(&self, character: char) -> Option<CharInfo> {
    let hex = utils::get_hex(character);
    let cns_code = self.unicode_to_cns.get(&hex)?;

    let char_info = self.global_map.get(cns_code)?;
    Some((*char_info).clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn zh_char_infos() {
    let data = OpenData::build(&"data".to_string());
    let infos = data.get_char_infos('我').unwrap();
    assert_eq!(infos.character, '我');
    assert_eq!(infos.cns_code, "1-4A3C");
    assert_eq!(infos.components.len(), 1);
    assert_eq!(infos.phonetic.len(), 2);
    assert_eq!(infos.strokes.total, 7);
  }

  #[test]
  fn zh_char_infos_not_found() {
    let data = OpenData::build(&"data".to_string());
    let infos = data.get_char_infos('x');
    assert!(infos.is_none());
  }
}
