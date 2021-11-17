//! API entry for Chinese characters Open Data

use once_cell::sync::OnceCell;
use std::collections::HashMap;
use types::{CharInfo, CnsCode, UnicodeHexVal};

pub mod filters;
mod loader;
mod types;
mod utils;

/// Shared Open Data
pub static DATA: OnceCell<OpenData> = OnceCell::new();

/// Main data structures
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
