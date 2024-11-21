use std::{
    fs::{self, File},
    io::{copy, Cursor, Error, ErrorKind},
};

use crate::{
    config::{read_config, write_config},
    constants::PYTHON_PACKAGES_DIR,
};
use serde::Deserialize;
use ureq::Response;
use zip::ZipArchive;

// TODO: use the version
pub fn add_packages(name: &String, _version: Option<&String>) -> Result<(), Error> {
    let pypi_url = format!("https://pypi.org/pypi/{name}/json");
    let response: Response = ureq::get(&pypi_url)
        .set("Accept", "application/json")
        .call()
        .unwrap();

    if response.status() != 200 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Request failed with status: {}", response.status()),
        ));
    }

    // Parse the response as JSON if expected
    let json: ApiResponse = response.into_json()?;
    // println!("Got response: {:?}", json);
    println!("Downloading: {} v{}", json.info.name, json.info.version);

    let response: Response = ureq::get(&json.urls[0].url).call().unwrap();

    if response.status() != 200 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Request failed with status: {}", response.status()),
        ));
    }

    let mut buffer = Vec::new();
    response.into_reader().read_to_end(&mut buffer)?;
    let mut zip = ZipArchive::new(Cursor::new(buffer))?;

    let extract_dir = PYTHON_PACKAGES_DIR.join(format!("{}_{}", json.info.name, json.info.version));
    fs::create_dir_all(&extract_dir)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let out_path = extract_dir.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = File::create(&out_path)?;
            copy(&mut file, &mut out_file)?;
        }
    }

    let mut config = read_config().unwrap();
    config
        .packages
        .insert(json.info.name, toml::Value::String(json.info.version));
    let _ = write_config(config);

    Ok(())
}

// See: https://warehouse.pypa.io/api-reference/json.html#get--pypi--project_name--json
#[derive(Deserialize, Debug)]
struct ApiResponse {
    info: PackageInfo,
    urls: Vec<PackageVersionUrl>,
}

#[derive(Deserialize, Debug)]
struct PackageInfo {
    name: String,
    summary: String,
    version: String,
    yanked: bool,
    yanked_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PackageVersionUrl {
    comment_text: String,
    digests: UrlDigests,
    filename: String,
    md5_digest: String,
    packagetype: String,
    requires_python: Option<String>,
    upload_time_iso_8601: String,
    url: String,
    yanked: bool,
    yanked_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UrlDigests {
    blake2b_256: String,
    md5: String,
    sha256: String,
}
