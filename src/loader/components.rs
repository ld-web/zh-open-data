//! Implementation for CNS code to components file(s)

use std::collections::HashMap;
use std::io;

use crate::types::{CharInfo, CnsCode, Components};

use super::Loader;

const CNS_TO_COMPONENT_FILES: [&str; 1] = ["data/CNS_component.txt"];

struct ComponentsLoader;

impl ComponentsLoader {
  /// Parse a given line a return a tuple containing extracted informations
  fn parse_line(line: String) -> Option<(CnsCode, Components)> {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() != 2 {
      return None;
    }
    let cns_code = s[0].to_string();
    let components_string = s[1].to_string();
    let components_sets: Vec<&str> = components_string.split(';').collect();
    let mut components_final: Components = Vec::new();

    for set in components_sets {
      let components: Vec<&str> = set.split(',').collect();
      components_final.push(components.iter().map(|c| c.to_string()).collect());
    }

    Some((cns_code, components_final))
  }
}

impl Loader<CnsCode, CharInfo> for ComponentsLoader {
  /// Process a line for a HashMap<CnsCode, CharInfo>
  ///
  /// So, put components into an existing CharInfo, found into the given map
  fn process_line(map: &mut HashMap<CnsCode, CharInfo>, line: String) {
    let (cns_code, components) = Self::parse_line(line).unwrap();

    if let Some(char_info) = map.get_mut(&cns_code) {
      char_info.components = components;
    }
  }
}

/// Load and map all components into an existing HashMap
pub fn load_into(map: &mut HashMap<CnsCode, CharInfo>) -> Result<(), io::Error> {
  let loader = ComponentsLoader {};

  loader.load_into_map(map, &CNS_TO_COMPONENT_FILES);

  Ok(())
}
