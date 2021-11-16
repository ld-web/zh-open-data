use clap::{crate_version, Parser};
use once_cell::sync::OnceCell;
use serde_derive::Serialize;
use warp::Filter;
use zh_open_data::{types::CharInfo, OpenData};

/// Options structure
#[derive(Parser)]
#[clap(version = crate_version!())]
struct Opts {
  /// Sets a custom load directory
  #[clap(short, long, default_value = "data")]
  pub load_dir: String,
}

#[derive(Serialize)]
enum LookupResult {
  Found(CharInfo),
  NotFound(String),
}

#[tokio::main]
async fn main() {
  let opts = Opts::parse();
  static DATA: OnceCell<OpenData> = OnceCell::new();
  DATA.get_or_init(|| OpenData::build(&opts.load_dir));

  let lookup = warp::path!("lookup" / String).map(|input: String| {
    let mut result: Vec<LookupResult> = Vec::new();
    let input_decoded = urlencoding::decode(input.as_str()).unwrap();
    let chars: Vec<char> = input_decoded.chars().collect();
    let data = DATA.get().unwrap();

    for char in chars {
      if let Some(char_infos) = data.get_char_infos(char) {
        result.push(LookupResult::Found(char_infos));
      } else {
        result.push(LookupResult::NotFound(format!("{}", char)));
      }
    }

    warp::reply::json(&result)
  });

  warp::serve(lookup).run(([127, 0, 0, 1], 8900)).await;
}
