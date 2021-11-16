//! Implementation for CNS code to stroke info file(s)

use std::collections::HashMap;
use std::io;

use crate::types::{CharInfo, CnsCode, StrokeInfo};

use super::{Loader, PathResolver};

struct StrokeLoader {}

impl StrokeLoader {
  /// Parse a given line a return a tuple containing extracted informations
  fn parse_line(line: String) -> Option<(CnsCode, StrokeInfo)> {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() != 2 {
      return None;
    }
    let cns_code = s[0].to_string();
    let stroke_count: u8 = s[1].to_string().parse().unwrap();
    let stroke_info = StrokeInfo {
      total: stroke_count,
    };

    Some((cns_code, stroke_info))
  }
}

impl Loader<CnsCode, CharInfo> for StrokeLoader {
  fn process_line(map: &mut HashMap<CnsCode, CharInfo>, line: String) {
    let (cns_code, stroke_info) = Self::parse_line(line).unwrap();

    if let Some(char_info) = map.get_mut(&cns_code) {
      char_info.strokes = stroke_info;
    }
  }
}

impl PathResolver for StrokeLoader {}

const CNS_TO_STROKE_COUNT_FILES: [&str; 1] = ["CNS_stroke.txt"];

/// Load and map all stroke counts into an existing HashMap
pub fn load_into(load_dir: &str, map: &mut HashMap<CnsCode, CharInfo>) -> Result<(), io::Error> {
  let loader = StrokeLoader {};
  let file_paths = loader.get_files_paths(load_dir, &CNS_TO_STROKE_COUNT_FILES);

  loader.load_into_map(map, &file_paths);

  Ok(())
}
