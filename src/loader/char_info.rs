//! Character informations loader

use super::{components, phonetic, stroke, unicode_to_cns};
use crate::types::{CharInfo, CnsCode};
use anyhow::Result;
use std::collections::HashMap;

/// Loads the global map containing all characters informations, each merged into a CharInfo struct
pub fn get_global_map(load_dir: &str) -> Result<HashMap<CnsCode, CharInfo>> {
  let mut global_map: HashMap<CnsCode, CharInfo> = HashMap::new();

  unicode_to_cns::load_into(load_dir, &mut global_map)?;
  components::load_into(load_dir, &mut global_map)?;
  phonetic::load_into(load_dir, &mut global_map)?;
  stroke::load_into(load_dir, &mut global_map)?;

  Ok(global_map)
}
