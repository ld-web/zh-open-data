//! General/Utility functions

/// Returns the unicode hex value for a given character, as a string.
///
/// Example :
/// ```ignore
/// let hex_code = get_hex('我');
/// assert_eq!(hex_code, "6211");
/// ```
pub fn get_hex(c: char) -> String {
  let hex = format!("{:x}", c as u32).to_uppercase();
  hex
}

/// Returns the character for a given unicode hexadecimal value
///
/// Example :
/// ```ignore
/// let character = get_char("6211");
/// assert_eq!(character, '我');
/// ```
pub fn get_char(hex_val: String) -> char {
  if let Ok(int_val) = u32::from_str_radix(hex_val.as_str(), 16) {
    let character = char::from_u32(int_val).unwrap();
    character
  } else {
    return '-';
  }
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

  #[test]
  fn hex_to_zh_char() {
    assert_eq!(get_char(String::from("6211")), '我');
    assert_eq!(get_char(String::from("807D")), '聽');
    assert_eq!(get_char(String::from("6240")), '所');
  }
}
