use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use saphyr::{LoadableYamlNode, Scalar, Yaml, YamlEmitter};
use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();
    let docs = Yaml::load_from_str(&buffer).unwrap();
    for mut doc in docs {
        let mdoc = doc.as_mapping_mut().unwrap();
        let data = Yaml::value_from_str("data");
        if let Some(mdata) = mdoc.get_mut(&data) {
            let mdata_mapping = mdata.as_mapping_mut().unwrap();
            for value in mdata_mapping.values_mut() {
                let clear_val = BASE64_STANDARD.decode(value.as_str().unwrap()).unwrap();
                let string_val = String::from_utf8(clear_val).unwrap();
                let ydata = Yaml::Value(Scalar::String(string_val.into()));
                *value = ydata;
            }
        }
        let mut output = String::new();
        YamlEmitter::new(&mut output).dump(&doc).unwrap();
        println!("{}", output);
    }
}
