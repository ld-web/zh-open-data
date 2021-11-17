//! API endpoints

use serde_derive::{Deserialize, Serialize};
use warp::Filter;

use crate::types::CharInfo;

/// Possible lookup results
#[derive(Serialize, Deserialize, Debug)]
pub enum LookupResult {
  /// If found, will contain the character informations
  Found(CharInfo),
  /// If not found, will contain a message
  NotFound(String),
}

/// Lookup a character or a series of characters into Open Data informations
///
/// Example :
///
/// `/lookup/你是哪國人`
pub fn lookup() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("lookup" / String).map(move |input: String| {
    let mut result: Vec<LookupResult> = Vec::new();
    let input_decoded = urlencoding::decode(input.as_str()).unwrap();
    let chars: Vec<char> = input_decoded.chars().collect();
    let data = crate::DATA.get().unwrap();

    for char in chars {
      if let Some(char_infos) = data.get_char_infos(char) {
        result.push(LookupResult::Found(char_infos));
      } else {
        result.push(LookupResult::NotFound(format!("{}", char)));
      }
    }

    warp::reply::json(&result)
  })
}
