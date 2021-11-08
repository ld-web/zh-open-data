//! Implementation for CNS code to stroke info file(s)

use std::collections::HashMap;
use std::io;

use crate::types::{CnsCode, StrokeInfo};

use super::Loader;

struct StrokeLoader {}

const CNS_TO_STROKE_COUNT_FILES: [&str; 1] = ["data/CNS_stroke.txt"];

impl Loader<CnsCode, StrokeInfo> for StrokeLoader {
  fn process_line(map: &mut HashMap<CnsCode, StrokeInfo>, line: String) {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() == 2 {
      let cns_code = s[0].to_string();
      let stroke_count: u8 = s[1].to_string().parse().unwrap();
      let stroke_info = StrokeInfo {
        total: stroke_count,
      };
      map.insert(cns_code, stroke_info);
    }
  }
}

/// This function will give a HashMap with CNS Code as keys and stroke infos as values
pub fn load() -> Result<HashMap<CnsCode, StrokeInfo>, io::Error> {
  let loader = StrokeLoader {};
  let data = loader.load_files_to_map(&CNS_TO_STROKE_COUNT_FILES)?;
  Ok(data)
}
