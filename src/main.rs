use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use saphyr::{LoadableYamlNode, Yaml, YamlEmitter};
use std::io;
use std::io::Read;
fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();
    let docs = Yaml::load_from_str(&buffer).unwrap();
    for doc in docs {
        let mdoc = doc.as_mapping().unwrap();
        let data = Yaml::load_from_str("data").unwrap();
        if let Some(mdata) = mdoc.get(&data[0]) {
            for (key, value) in mdata.as_mapping().unwrap() {
                let clear_key = BASE64_STANDARD.decode(value.as_str().unwrap()).unwrap();
                let string_key = String::from_utf8(clear_key).unwrap();
                println!("{}: {}", key.as_str().unwrap(), string_key)
            }
        }
    }
}
