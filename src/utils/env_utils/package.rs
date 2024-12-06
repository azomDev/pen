use crate::utils::{self, error, guard, AnyError};
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::{
	collections::HashMap,
	env::consts::{ARCH, OS},
	fs::{self, File},
	io::{copy, Cursor},
};
use zip::ZipArchive;

// todo docstring
pub fn download_package(package: &Package, py_version: &Version) -> Result<(), AnyError> {
	println!("Downloading: {} v{}", package.name, package.version);
	let url = match find_package_download_url(package, py_version)? {
		Some(url) => url,
		None => return error!("This package doesn't seem compatible with your os."),
	};
	// let request = minreq::get(&url).with_header("Accept", "application/json");
	// let response = guard!(request.send(), "Couldn't request PyPi");
	let request = ureq::get(&url).set("Accept", "application/json");
	let response = guard!(request.call(), "Couldn't request PyPi");

	if response.status() != 200 {
		return error!("Package download request failed with status: {}.", response.status());
	}

	let mut buffer = Vec::new();
	guard!(response.into_reader().read_to_end(&mut buffer), "Couldn't read the body of the response.");
	let mut zip = guard!(ZipArchive::new(Cursor::new(buffer)), "Couldn't uncompress {}.", package.name);

	let extract_dir = utils::get_package_path(&package);
	guard!(fs::create_dir_all(&extract_dir), "Couldn't create folder.");
	for i in 0..zip.len() {
		let mut file = zip.by_index(i).expect("File count changed while iterating.");
		let out_path = extract_dir.join(file.name());

		if file.is_dir() {
			guard!(fs::create_dir_all(&out_path), "Couldn't create folder.");
		} else {
			if let Some(parent) = out_path.parent() {
				guard!(fs::create_dir_all(parent), "Couldn't create folder.");
			}

			let mut out_file = guard!(File::create(&out_path), "Couldn't create {}.", out_path.display());
			guard!(copy(&mut file, &mut out_file), "Couldn't write {} to disk.", out_path.display());
		}
	}
	return Ok(());
}

// todo docstring
pub fn find_matching_package_version(name: &str, version_requirements: &VersionReq) -> Result<Package, AnyError> {
	let request = ureq::get(&format!("https://pypi.org/pypi/{}/json", name)).set("Accept", "application/json");
	let response = guard!(request.call(), "Couldn't request PyPi.");

	if response.status() != 200 {
		return error!("Package info request failed with status: {}.", response.status());
	}

	// Parse the response as JSON if expected
	let json = guard!(response.into_json::<ApiPackageResponse>(), "Received an invalid response from PyPi.");

	let best_version = match json
		.releases
		.iter()
		.filter_map(|(version, vec)| {
			Some(version)
				.filter(|_| !vec.is_empty())
				.and_then(|v| Version::parse(&v).ok().filter(|v| version_requirements.matches(&v)))
		})
		.max()
	{
		Some(version) => version,
		None => todo!("No version matched {} for package {}.", version_requirements, name),
	};

	return Ok(Package {
		name: String::from(&json.info.name), // todo why is this not just name?
		version: best_version,
	});
}

// todo docstring
fn find_package_download_url(package: &Package, py_version: &Version) -> Result<Option<String>, AnyError> {
	let request = ureq::get(&format!("https://pypi.org/pypi/{}/{}/json", package.name, package.version)).set("Accept", "application/json");
	let response = guard!(request.call(), "Couldn't request PyPi.");

	if response.status() != 200 {
		return error!("Package info request failed with status: {}.", response.status());
	}

	// Parse the response as JSON if expected
	let json = guard!(
		response.into_json::<ApiPackageVersionResponse>(),
		"Received an invalid response from PyPi"
	);

	let python = format!("py{}{}", py_version.major, py_version.minor);
	let arch = match OS {
		"macos" if ARCH == "aarch64" => "arm64",
		_ => ARCH,
	};

	let idk = json
		.urls
		.iter()
		.max_by(|a, b| {
			let a = a.url.split("-").collect::<Vec<&str>>();
			let b = b.url.split("-").collect::<Vec<&str>>();

			(a.len() > 3).cmp(&(b.len() > 3)).then_with(|| {
				let wheel = a[a.len() - 1].ends_with(".whl").cmp(&b[b.len() - 1].ends_with(".whl"));
				let python = a[a.len() - 3].contains(&python).cmp(&b[b.len() - 3].contains(&python));
				let arch = a[a.len() - 1].contains(arch).cmp(&b[b.len() - 1].contains(arch));
				let os = a[a.len() - 1].contains(OS).cmp(&b[b.len() - 1].contains(OS));

				wheel.then(python).then(arch).then(os)
			})
		})
		.map(|p| p.url.clone());
	return Ok(idk);
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
