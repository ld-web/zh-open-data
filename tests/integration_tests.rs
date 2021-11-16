use zh_open_data::OpenData;

#[test]
fn zh_char_infos() {
  let data = OpenData::build(&"tests/data".to_string());
  let infos = data.get_char_infos('我').unwrap();
  assert_eq!(infos.character, '我');
  assert_eq!(infos.cns_code, "1-4A3C");
  assert_eq!(infos.components.len(), 1);
  assert_eq!(infos.phonetic.len(), 2);
  assert_eq!(infos.strokes.total, 7);
}

#[test]
fn zh_char_infos_not_found() {
  let data = OpenData::build(&"tests/data".to_string());
  let infos = data.get_char_infos('x');
  assert!(infos.is_none());
}
