use std::{path::Path, io::Read};
use config::Config;

use crate::conf::ReboundConf;


pub fn parse(file: String) -> ReboundConf {
    let conf = Config::builder()
        .add_source(config::File::from(Path::new(file.as_str())))
        .build()
        .unwrap();

    conf.try_deserialize::<ReboundConf>().unwrap()
}

pub fn read_ssl_file(file: String) -> Vec<u8> {
    let mut f = std::fs::File::open(&file).expect("no file found");
    let metadata = std::fs::metadata(&file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("failed to read to buffer");
    buffer
}