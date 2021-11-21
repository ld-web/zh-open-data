use clap::{crate_version, Parser};
use zh_open_data::{filters, OpenData};

/// Options structure
#[derive(Parser)]
#[clap(version = crate_version!())]
struct Opts {
  /// Sets a custom load directory
  #[clap(short, long, default_value = "data")]
  pub load_dir: String,
}

#[tokio::main]
async fn main() {
  let opts = Opts::parse();
  zh_open_data::DATA.get_or_init(|| OpenData::build(&opts.load_dir));

  warp::serve(filters::lookup())
    .run(([0, 0, 0, 0], 8900))
    .await;
}
