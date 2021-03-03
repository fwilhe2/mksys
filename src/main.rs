use std::fs;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

fn main() {

    let contents = fs::read_to_string("config.yaml").expect("Something went wrong reading the file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    println!("{:#?}", doc);

    let packages = &doc["packages"];
    let apt = &packages["apt"];
    let dnf = &packages["dnf"];
    let directories = &doc["directories"];

    println!("{:#?}", apt);
    println!("{:#?}", dnf);

    println!("{:#?}", directories);

}