use warp::http::StatusCode;
use warp::test::request;
use zh_open_data::OpenData;

use zh_open_data::filters::{self, LookupResult};

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

#[tokio::test]
async fn test_get() {
  zh_open_data::DATA.get_or_init(|| OpenData::build(&"tests/data".to_string()));

  let api = filters::lookup();
  let input = urlencoding::encode("你很屌").into_owned();

  let resp = request()
    .method("GET")
    .path(&format!("/{}/{}", "lookup", input))
    .reply(&api)
    .await;

  let body_string = String::from_utf8(resp.body().to_vec()).unwrap();
  let results: Vec<LookupResult> = serde_json::from_str(&body_string).unwrap();

  assert_eq!(resp.status(), StatusCode::OK);
  assert_eq!(results.len(), 3);

  for result in results {
    assert!(matches!(result, LookupResult::Found(..)));
  }
}
