//! Custom types and alias

pub type UnicodeHexVal = String;
pub type CnsCode = String;
pub type Components = Vec<Vec<String>>;
pub type Phonetic = Vec<String>;

#[derive(Debug, Copy, Clone)]
pub struct StrokeInfo {
  pub total: u8,
}

#[derive(Debug)]
pub struct CharInfo {
  pub character: char,
  pub cns_code: CnsCode,
  pub components: Components,
  pub phonetic: Phonetic,
  pub strokes: StrokeInfo,
}
