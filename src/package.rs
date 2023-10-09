use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PackageResponse {
    // _id: String,
    // _rev: String,
    // name: String,
    // #[serde(rename = "dist-tags")]
    // dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, PackageVersion>,
    // time: Box<RawValue>,
    // maintainers: Box<RawValue>,
    // description: String,
    // homepage: String,
    // keywords: Box<RawValue>,
    // repository: Box<RawValue>,
    // author: Box<RawValue>,
    // bugs: Box<RawValue>,
    // license: String,
    // readme: String,
    // #[serde(rename = "readmeFilename")]
    // readme_filename: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageVersion {
    // pub name: String,
    // version: String,
    // description: String,
    // main: String,
    // typings: String,
    // module: String,
    // scripts: HashMap<String, String>,
    pub dist: PackageVersionDist,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageVersionDist {
    pub integrity: Option<String>,
    pub shasum: Option<String>,
    pub tarball: Option<String>,
    #[serde(rename = "fileCount")]
    pub file_count: Option<u16>,
    #[serde(rename = "unpackedSize")]
    pub unpacked_size: Option<u64>,
}
