//! API entry for Chinese characters Open Data

use types::CharInfo;

mod data;
mod loader;
mod opts;
mod types;
mod utils;

/// Get a single character infos
/// ```rust
/// use zh_open_data::get_char_infos;
/// let infos = get_char_infos('我').unwrap();
/// ```
pub fn get_char_infos(character: char) -> Option<CharInfo> {
  let hex = utils::get_hex(character);
  let cns_code = data::UNICODE_TO_CNS.get(&hex)?;

  let char_info = data::GLOBAL_MAP.get(cns_code)?;
  Some((*char_info).clone())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn zh_char_infos() {
    let infos = get_char_infos('我').unwrap();
    assert_eq!(infos.character, '我');
    assert_eq!(infos.cns_code, "1-4A3C");
    assert_eq!(infos.components.len(), 1);
    assert_eq!(infos.phonetic.len(), 2);
    assert_eq!(infos.strokes.total, 7);
  }

  #[test]
  fn zh_char_infos_not_found() {
    let infos = get_char_infos('x');
    assert!(infos.is_none());
  }
}
