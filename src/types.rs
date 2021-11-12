//! Custom types and alias

/// Unicode hexadecimal representation
pub type UnicodeHexVal = String;

/// CNS (Chinese National Standard) Code from Open Data files
pub type CnsCode = String;

/// Possible components combinations
pub type Components = Vec<Vec<String>>;

/// Possible phonetics, in Zhuyin (注音)
pub type Phonetic = Vec<String>;

/// Stroke informations. Meant to also contain stroke sequence in the future
#[derive(Debug, Copy, Clone, Default)]
pub struct StrokeInfo {
  pub total: u8,
}

/// All character informations
#[derive(Debug, Clone)]
pub struct CharInfo {
  pub character: char,
  pub cns_code: CnsCode,
  pub components: Components,
  pub phonetic: Phonetic,
  pub strokes: StrokeInfo,
}

impl CharInfo {
  /// Default constructor for a given char & CNS Code
  pub fn new(c: char, cns_code: CnsCode) -> Self {
    CharInfo {
      character: c,
      cns_code,
      components: vec![],
      phonetic: vec![],
      strokes: Default::default(),
    }
  }
}
