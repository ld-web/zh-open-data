//! Static hash maps with open data

use crate::loader::{char_info, unicode_to_cns};
use crate::types::{CharInfo, CnsCode, UnicodeHexVal};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  /// A HashMap containing unicode hexadecimal values as keys and corresponding CNS Codes as values, for each character
  pub static ref UNICODE_TO_CNS: HashMap<UnicodeHexVal, CnsCode> = unicode_to_cns::get_single_map().unwrap();
  /// A global HashMap of all CNS Codes and the corresponding character informations
  pub static ref GLOBAL_MAP: HashMap<CnsCode, CharInfo> = char_info::load_global_map().unwrap();
}
