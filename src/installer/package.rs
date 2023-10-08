use std::collections::HashMap;

use serde::Deserialize;
use serde_json::value::RawValue;

#[derive(Debug, Deserialize)]
pub struct PackageResponse {
    _id: String,
    _rev: String,
    name: String,
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, HashMap<String, Box<RawValue>>>,
    time: Box<RawValue>,
    maintainers: Box<RawValue>,
    description: String,
    homepage: String,
    keywords: Box<RawValue>,
    repository: Box<RawValue>,
    author: Box<RawValue>,
    bugs: Box<RawValue>,
    license: String,
    readme: String,
    #[serde(rename = "readmeFilename")]
    readme_filename: String,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    pub name: String,
    version: String,
    description: String,
    main: String,
    typings: String,
    module: String,
    scripts: HashMap<String, String>,
    dist: HashMap<String, Box<RawValue>>,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersionDist {
    integrity: String,
    shasum: String,
    tarball: String,
    #[serde(rename = "fileCount")]
    file_count: usize,
    #[serde(rename = "unpackedSize")]
    unpacked_size: u64,
}
