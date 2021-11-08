//! Implementation for CNS code to components file(s)

use std::collections::HashMap;
use std::io;

use crate::types::{CnsCode, Components};

use super::Loader;

const CNS_TO_COMPONENT_FILES: [&str; 1] = ["data/CNS_component.txt"];

struct ComponentsLoader {}

impl Loader<CnsCode, Components> for ComponentsLoader {
  fn process_line(map: &mut HashMap<CnsCode, Components>, line: String) {
    let s: Vec<&str> = line.split('\t').collect();
    if s.len() == 2 {
      let cns_code = s[0].to_string();
      let components_string = s[1].to_string();
      let components_sets: Vec<&str> = components_string.split(';').collect();
      let mut output_val: Components = Vec::new();

      for set in components_sets {
        let components: Vec<&str> = set.split(',').collect();
        output_val.push(components.iter().map(|c| c.to_string()).collect());
      }
      map.insert(cns_code, output_val);
    }
  }
}

/// This function will give a HashMap with CNS Code as keys and possible components sets as values
pub fn load() -> Result<HashMap<CnsCode, Components>, io::Error> {
  let loader = ComponentsLoader {};
  let data = loader.load_files_to_map(&CNS_TO_COMPONENT_FILES)?;
  Ok(data)
}
