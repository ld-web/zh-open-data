//! General/Utility functions

/// Returns the unicode hex value for a given character, as a string.
pub fn get_hex(c: char) -> String {
  let hex = format!("{:x}", c as u32).to_uppercase();
  hex
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn zh_chars_to_hex() {
    let hex_code_1 = get_hex('我');
    assert_eq!(hex_code_1, "6211");

    let hex_code_2 = get_hex('聽');
    assert_eq!(hex_code_2, "807D");

    let hex_code_3 = get_hex('所');
    assert_eq!(hex_code_3, "6240");
  }
}
