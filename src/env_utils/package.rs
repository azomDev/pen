use std::{
	collections::HashMap,
	env::consts::{ARCH, OS},
	fs::{self, File},
	io::{copy, Cursor},
};

use semver::{Version, VersionReq};
use serde::Deserialize;
use zip::ZipArchive;

use crate::utils::{self, abort};

// todo docstring
pub fn download_package(package: &Package, py_version: &Version) {
	println!("Downloading: {} v{}", package.name, package.version);
	let url = match find_package_download_url(package, py_version) {
		Some(url) => url,
		None => abort("This package doesn't seem compatible with your os.", None),
	};

	let response = match ureq::get(&url).set("Accept", "application/json").call() {
		Ok(res) => res,
		Err(e) => abort("Couldn't request PyPi", Some(&e)),
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
		Err(e) => abort(&format!("Couldn't uncompress {}.", package.name), Some(&e)),
	};

	let extract_dir = utils::get_package_path(&package);
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
}

// todo docstring
pub fn find_matching_package_version(name: &str, version_requirements: &VersionReq) -> Package {
	let response = match ureq::get(&format!("https://pypi.org/pypi/{}/json", name))
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
	let json = match response.into_json::<ApiPackageResponse>() {
		Ok(json) => json,
		Err(e) => abort("Received an invalid response from PyPi.", Some(&e)),
	};

	let best_version = match json
		.releases
		.iter()
		.filter_map(|(version, vec)| {
			Some(version).filter(|_| !vec.is_empty()).and_then(|v| {
				Version::parse(&v)
					.ok()
					.filter(|v| version_requirements.matches(&v))
			})
		})
		.max()
	{
		Some(version) => version,
		None => abort(
			&format!(
				"No version matched {} for package {}.",
				version_requirements, name
			),
			None,
		),
	};

	Package {
		name: String::from(&json.info.name),
		version: best_version,
	}
}

// todo docstring
fn find_package_download_url(package: &Package, py_version: &Version) -> Option<String> {
	let response = match ureq::get(&format!(
		"https://pypi.org/pypi/{}/{}/json",
		package.name, package.version
	))
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
	let json = match response.into_json::<ApiPackageVersionResponse>() {
		Ok(json) => json,
		Err(e) => abort("Received an invalid response from PyPi.", Some(&e)),
	};

	let python = format!("py{}{}", py_version.major, py_version.minor);
	let arch = match OS {
		"macos" if ARCH == "aarch64" => "arm64",
		_ => ARCH,
	};
	json.urls
		.iter()
		.max_by(|a, b| {
			let a = a.url.split("-").collect::<Vec<&str>>();
			let b = b.url.split("-").collect::<Vec<&str>>();

			(a.len() > 3).cmp(&(b.len() > 3)).then_with(|| {
				let wheel = a[a.len() - 1]
					.ends_with(".whl")
					.cmp(&b[b.len() - 1].ends_with(".whl"));
				let python = a[a.len() - 3]
					.contains(&python)
					.cmp(&b[b.len() - 3].contains(&python));
				let arch = a[a.len() - 1]
					.contains(arch)
					.cmp(&b[b.len() - 1].contains(arch));
				let os = a[a.len() - 1]
					.contains(OS)
					.cmp(&b[b.len() - 1].contains(OS));

				wheel.then(python).then(arch).then(os)
			})
		})
		.map(|p| p.url.clone())
}

pub struct Package {
	pub name: String,
	pub version: Version,
}

// See: https://warehouse.pypa.io/api-reference/json.html#get--pypi--project_name--json
#[derive(Deserialize, Debug)]
struct ApiPackageResponse {
	info: PackageInfo,
	releases: HashMap<String, Vec<PackageVersionUrl>>,
	urls: Vec<PackageVersionUrl>,
}

#[derive(Deserialize, Debug)]
struct ApiPackageVersionResponse {
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
