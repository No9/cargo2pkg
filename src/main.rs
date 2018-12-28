extern crate clap;
#[macro_use]
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate sha2;

use clap::App;
use serde_json::Value as Json;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use toml::Value as Toml;

fn main() {
    App::new(
        "cargo2pkg - a cli for generating FreeBSD pkg manifests and artifacts from Cargo.toml",
    )
    .version("1.0")
    .author("Anton Whalley <anton@venshare.com>")
    .about("Run in the root of your cargo project")
    .get_matches();

    match generate_json() {
        Ok(pkg) => {
            println!("{}", serde_json::to_string_pretty(&pkg).unwrap());
        }
        Err(e) => {
            panic!(e);
        }
    }
}

fn generate_json() -> Result<PkgManifest, io::Error> {
    let mut input = String::new();
    File::open("Cargo.toml")
        .and_then(|mut f| f.read_to_string(&mut input))
        .unwrap();

    match input.parse() {
        Ok(toml) => {
            let json = convert(toml);
            let obj = json.as_object().unwrap();

            let mut path = "./target/release/".to_string();
            path.push_str(&String::from(obj["package"]["name"].as_str().unwrap()));
            let fileloc = path.clone();
            let metadata = fs::metadata(path).unwrap();

            let mut file = File::open(fileloc).unwrap();
            let mut sha256 = Sha256::new();
            io::copy(&mut file, &mut sha256).unwrap();
            let hash = sha256.result();

            let hx = hex::encode(hash);
            let files = json!({
            obj["package"]["name"].as_str().unwrap() : &String::from(hx)[..]
            });
            let pkg = PkgManifest {
                name: String::from(obj["package"]["name"].as_str().unwrap()),
                origin: String::from(""),
                version: String::from(obj["package"]["version"].as_str().unwrap()),
                comment: String::from(""),
                maintainer: String::from(obj["package"]["authors"][0].as_str().unwrap()),
                abi: String::from(""),
                arch: String::from(""),
                prefix: String::from(""),
                flatsize: u64::from(metadata.len()),
                desc: String::from(""),
                files: files,
            };
            Ok(pkg)
        }
        Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
    }
}

#[test]
fn test_generate_json() {
    match generate_json() {
        Ok(pkg) => {
            assert_eq!(pkg.name, "cargo2pkg");
            assert_eq!(pkg.version, "0.1.0");
            assert_eq!(pkg.maintainer, "Anton Whalley <anton@venshare.com>");
            assert!(pkg.flatsize > 0);
        }
        Err(e) => {
            panic!(e);
        }
    }
}

fn convert(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(convert).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, convert(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}
#[derive(Serialize, Deserialize)]
struct PkgManifest {
    name: String,
    origin: String,
    version: String,
    comment: String,
    maintainer: String,
    abi: String,
    arch: String,
    prefix: String,
    flatsize: u64,
    desc: String,
    files: serde_json::Value,
}
