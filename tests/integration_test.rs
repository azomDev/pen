use home;
use pen::utils;

#[test]
fn get_version_paths() {
    let home_dir = home::home_dir().expect("Failed to get home directory");

    // A list of versions to test
    let versions = vec!["3.11.9", "3.10.8", "3.9.7", "3.8.10", "3.7.9"];

    // Loop through the versions and test each one
    for version in versions {
        let expected_path = home_dir.join(format!(".pen/python_versions/python_{}", version));
        let version_path = utils::get_version_path(version);
        assert_eq!(version_path, expected_path, "Version mismatch for {}", version);
    }
}


#[test]
fn other_test() {
    // todo other tests
}
