#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;
use clap::App;

use std::path::Path;
use std::fs;


fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let output_dir = Path::new({
        if matches.is_present("outputDir") {
            matches.value_of("outputDir").unwrap()
        } else {
            "./offline-website"
        }
    });

    if output_dir.exists() == false {
        match fs::create_dir_all(output_dir) {
            Err(why) => error!("Couldn't create output directory {:?}", why.kind()),
            Ok(_) => warn!("{} doesn't exist so creating it", output_dir.display()),
        }
    }

}
