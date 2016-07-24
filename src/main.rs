#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;
use clap::App;

extern crate url;
use url::Url;

use std::path::Path;
use std::fs;


fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let output_dir = Path::new(matches.value_of("outputDir").unwrap_or("./offline-website"));

    if !output_dir.exists() {
        match fs::create_dir_all(output_dir) {
            Err(why) => error!("Couldn't create output directory {:?}", why.kind()),
            Ok(_) => warn!("{} doesn't exist so creating it", output_dir.display()),
        }
    }

    let parse_op = Url::parse(matches.value_of("url").unwrap());

    if parse_op.is_err() {
        error!("{:?}", parse_op.unwrap());
        return;
    }

    let base_url_to_crawl_from = parse_op.unwrap();
    info!("Crawler will begin downloading resources as it navigates from {}",
          base_url_to_crawl_from);

}
