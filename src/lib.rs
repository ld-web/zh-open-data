//! API entry for Chinese characters Open Data

use types::CharInfos;

mod data;
mod loader;
mod types;
mod utils;

/// Get a single character infos
/// ```rust
/// use zh_open_data::get_char_infos;
/// let infos = get_char_infos('我').unwrap();
/// ```
pub fn get_char_infos(character: char) -> Option<CharInfos> {
  let hex = utils::get_hex(character);

  // TODO: get rid of these ugly nested if /!\
  if let Some(cns_code) = data::UNICODE_TO_CNS.get(&hex) {
    if let Some(comps) = data::CNS_TO_COMPONENTS.get(cns_code) {
      if let Some(phonetic) = data::CNS_TO_PHONETIC.get(cns_code) {
        if let Some(stroke_info) = data::CNS_TO_STROKE_INFO.get(cns_code) {
          return Some(CharInfos {
            character,
            cns_code: cns_code.to_string(),
            components: comps.to_vec(),
            phonetic: phonetic.to_vec(),
            strokes: *stroke_info,
          });
        }
      }
    }
  }

  None
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
