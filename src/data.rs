//! Static hash maps with open data

use crate::loader::{components, phonetic, stroke, unicode_to_cns};
use crate::types::{CnsCode, Components, Phonetic, StrokeInfo, UnicodeHexVal};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  pub static ref UNICODE_TO_CNS: HashMap<UnicodeHexVal, CnsCode> = unicode_to_cns::load().unwrap();
  pub static ref CNS_TO_COMPONENTS: HashMap<CnsCode, Components> = components::load().unwrap();
  pub static ref CNS_TO_PHONETIC: HashMap<CnsCode, Phonetic> = phonetic::load().unwrap();
  pub static ref CNS_TO_STROKE_INFO: HashMap<CnsCode, StrokeInfo> = stroke::load().unwrap();
}
