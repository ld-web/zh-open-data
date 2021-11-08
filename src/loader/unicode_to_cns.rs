//! Implementation for CNS code to unicode file(s)

use std::collections::HashMap;
use std::io;

use crate::types::{CnsCode, UnicodeHexVal};

use super::Loader;

const CNS_TO_UNICODE_FILES: [&str; 3] = [
  "data/CNS2UNICODE_Unicode_2.txt",
  "data/CNS2UNICODE_Unicode_BMP.txt",
  "data/CNS2UNICODE_Unicode_15.txt",
];

struct UnicodeToCnsLoader {}

impl Loader<UnicodeHexVal, CnsCode> for UnicodeToCnsLoader {
  fn process_line(map: &mut HashMap<UnicodeHexVal, CnsCode>, line: String) {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() == 2 {
      let cns_code = s[0].to_string();
      let unicode = s[1].to_string();
      map.insert(unicode, cns_code);
    }
  }
}

/// This function will give a HashMap with unicode hex value as keys and CNS Code as values
pub fn load() -> Result<HashMap<UnicodeHexVal, CnsCode>, io::Error> {
  let loader = UnicodeToCnsLoader {};
  let data = loader.load_files_to_map(&CNS_TO_UNICODE_FILES)?;
  Ok(data)
}
