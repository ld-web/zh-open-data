//! Character informations loader

use std::collections::HashMap;
use std::io;

use crate::types::{CharInfo, CnsCode};

use super::{components, phonetic, stroke, unicode_to_cns};

/// Loads the global map containing all characters informations, each merged into a CharInfo struct
pub fn load_global_map() -> Result<HashMap<CnsCode, CharInfo>, io::Error> {
  let mut global_map: HashMap<CnsCode, CharInfo> = HashMap::new();

  unicode_to_cns::load_into(&mut global_map)?;
  components::load_into(&mut global_map)?;
  phonetic::load_into(&mut global_map)?;
  stroke::load_into(&mut global_map)?;

  Ok(global_map)
}
