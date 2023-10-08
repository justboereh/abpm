use std::collections::HashMap;

use clap::ArgMatches;

use package::PackageResponse;
use serde::{de, Deserializer};
use serde_json::value::RawValue;

mod package;

const PACKAGE_BASE_URL: &str = "https://registry.npmjs.org/";

pub async fn install(matches: &ArgMatches) {
    let packages = matches.get_many::<String>("packages").unwrap();

    for package in packages {
        let version_symbol_index = package.chars().skip(1).position(|c| c == '@');
        let mut version = String::from("latest");
        let mut package_name = String::from(package);

        if version_symbol_index != None {
            let v = package.chars().skip(version_symbol_index.unwrap() + 2);

            version = String::from_iter(v);
            package_name =
                String::from_iter(package.chars().take(version_symbol_index.unwrap() + 1))
        }

        let response =
            reqwest::get(String::from(PACKAGE_BASE_URL.to_owned() + &package_name)).await;

        if response.is_err() {
            return;
        }

        let data = response.unwrap().json::<PackageResponse>().await;

        if data.is_err() {
            println!("package errored: {} -- {:?}", package_name, data.err());

            return;
        }

        let versions = data.unwrap().versions;
        let last_version = versions
            .clone()
            .into_values()
            .skip(versions.clone().len() - 1)
            .next()
            .unwrap();

        println!("{:?}", last_version);
    }
}