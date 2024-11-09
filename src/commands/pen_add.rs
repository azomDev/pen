use std::{
    fs::{self, File},
    io::{copy, Cursor},
};

use crate::{
    config::{find_project, read_config, write_config},
    constants::PYTHON_PACKAGES_DIR,
    utils::abort,
};
use semver::Version;
use serde::Deserialize;
use zip::ZipArchive;

pub fn add(name: &String, version: Option<&String>) {
    let projet_path = find_project();
    let mut config = read_config(&projet_path);

    let info_url = match version {
        Some(version_str) => {
            let version = match Version::parse(version_str) {
                Ok(version) => version,
                Err(e) => abort("Provided version is invalid.", Some(&e)),
            };
            format!("https://pypi.org/pypi/{}/{}/json", name, version)
        }
        None => format!("https://pypi.org/pypi/{}/json", name),
    };

    let response = match ureq::get(&info_url)
        .set("Accept", "application/json")
        .call()
    {
        Ok(res) => res,
        Err(e) => abort("Couldn't request PyPi.", Some(&e)),
    };

    if response.status() != 200 {
        abort(
            &format!(
                "Package info request failed with status: {}.",
                response.status()
            ),
            None,
        );
    }

    // Parse the response as JSON if expected
    let json = match response.into_json::<ApiResponse>() {
        Ok(json) => json,
        Err(e) => abort("Received an invalid response from PyPi.", Some(&e)),
    };

    println!("Downloading: {} v{}", json.info.name, json.info.version);
    let response = match ureq::get(&json.urls[0].url)
        .set("Accept", "application/json")
        .call()
    {
        Ok(res) => res,
        Err(e) => abort("Couldn't request PyPi.", Some(&e)),
    };

    if response.status() != 200 {
        abort(
            &format!(
                "Package download request failed with status: {}.",
                response.status()
            ),
            None,
        );
    }

    let mut buffer = Vec::new();
    if let Err(e) = response.into_reader().read_to_end(&mut buffer) {
        abort("Couldn't read the body of the response.", Some(&e));
    }
    let mut zip = match ZipArchive::new(Cursor::new(buffer)) {
        Ok(zip) => zip,
        Err(e) => abort(
            &format!("Couldn't uncompress {}.", json.info.name),
            Some(&e),
        ),
    };

    let extract_dir = PYTHON_PACKAGES_DIR.join(format!("{}_{}", json.info.name, json.info.version));
    if let Err(e) = fs::create_dir_all(&extract_dir) {
        abort("Couldn't create folder.", Some(&e))
    }

    for i in 0..zip.len() {
        let mut file = zip
            .by_index(i)
            .expect("File count changed while iterating.");
        let out_path = extract_dir.join(file.name());

        if file.is_dir() {
            if let Err(e) = fs::create_dir_all(&out_path) {
                abort("Couldn't create folder.", Some(&e))
            }
        } else {
            if let Some(parent) = out_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    abort("Couldn't create folder.", Some(&e))
                }
            }

            match File::create(&out_path) {
                Ok(mut out_file) => {
                    if let Err(e) = copy(&mut file, &mut out_file) {
                        abort(
                            &format!("Couldn't write {} to disk.", out_path.display()),
                            Some(&e),
                        )
                    }
                }
                Err(e) => abort(
                    &format!("Couldn't create {}.", out_path.display()),
                    Some(&e),
                ),
            }
        }
    }

    config
        .packages
        .insert(json.info.name, toml::Value::String(json.info.version));
    write_config(projet_path, config);
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
