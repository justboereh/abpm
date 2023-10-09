use clap::ArgMatches;
use colored::*;
use reqwest::Response;

mod package;

const PACKAGE_BASE_URL: &str = "https://registry.npmjs.org/";

pub async fn install(matches: &ArgMatches) {
    let packages = matches.get_many::<String>("packages").unwrap();
    let _dev = matches.get_flag("dev");
    let _offline = matches.get_flag("offline");

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
            println!(
                "{} Fetching package: {} -- {:?}",
                " ERROR ".on_red().white(),
                package_name,
                response.err()
            );

            std::process::exit(0);
        }

        let dist = get_package_dist(response.unwrap(), &version).await;

        if dist.is_err() {
            println!(
                "{} Getting package dist: {} -- {:?}",
                " ERROR ".on_red().white(),
                package_name,
                dist.err()
            );

            std::process::exit(0);
        }

        println!("{:?}", dist.ok());
    }
}

async fn get_package_dist(
    res: Response,
    ver: &String,
) -> Result<package::PackageVersionDist, Box<dyn std::error::Error>> {
    let contents = res.text().await?;
    let response = serde_json::from_str::<package::PackageResponse>(&contents.as_str())?;
    let version = if ver.eq("latest") {
        let values = response.versions.values();

        values
            .clone()
            .skip(values.clone().len() - 1)
            .next()
            .unwrap()
    } else {
        response.versions.get(ver.as_str()).unwrap()
    };

    return Ok(version.dist.clone());
}
