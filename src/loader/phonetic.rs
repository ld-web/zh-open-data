//! Implementation for CNS code to phonetic file(s) (Zhuyin / 注音)

use std::collections::HashMap;
use std::io;

use crate::types::{CnsCode, Phonetic};

use super::Loader;

const CNS_TO_PHONETIC_FILES: [&str; 1] = ["data/CNS_phonetic.txt"];

struct PhoneticLoader {}

impl Loader<CnsCode, Phonetic> for PhoneticLoader {
  fn process_line(map: &mut HashMap<CnsCode, Phonetic>, line: String) {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() == 2 {
      let cns_code = s[0].to_string();
      let phonetic = s[1].to_string();
      if let Some(phonetic_set) = map.get_mut(&cns_code) {
        phonetic_set.push(phonetic);
      } else {
        let phonetics: Vec<String> = vec![phonetic];
        map.insert(cns_code, phonetics);
      }
    }
  }
}

/// This function will give a HashMap with CNS Code as keys and possible phonetics (Zhuyin / 注音) as values
pub fn load() -> Result<HashMap<CnsCode, Phonetic>, io::Error> {
  let loader = PhoneticLoader {};
  let data = loader.load_files_to_map(&CNS_TO_PHONETIC_FILES)?;
  Ok(data)
}
