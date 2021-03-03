extern crate yaml_rust;
use yaml_rust::{YamlLoader};

fn main() {
    let s =
"
apt:
  - tree
  - unzip
  - vim
  - zsh

dnf:
  - tree
  - unzip
  - vim
  - xz
  - zsh

directories:
  - ~/software
  - ~/bin
  - ~/code

";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    println!("{:#?}", doc);

    let apt = &doc["apt"];
    let dnf = &doc["dnf"];
    let directories = &doc["directories"];

    println!("{:#?}", apt);
    println!("{:#?}", dnf);

    println!("{:#?}", directories);

}