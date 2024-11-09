use home;
use pen::utils;
use semver::Version;

#[test]
fn get_version_paths() {
    let home_dir = home::home_dir().expect("Failed to get home directory");

    // A list of versions to test
    let versions = vec![
        Version::new(3, 11, 9),
        Version::new(3, 10, 8),
        Version::new(3, 9, 7),
        Version::new(3, 8, 10),
        Version::new(3, 7, 9),
    ];

    // Loop through the versions and test each one
    for version in versions {
        let expected_path = home_dir.join(format!(".cache/pen/python/python{}", version));
        let version_path = utils::get_python_path(&version);
        assert_eq!(
            version_path, expected_path,
            "Version mismatch for {}",
            version
        );
    }

    assert_eq!(
        utils::get_python_path(&Version::new(3, 11, 9)),
        utils::get_python_path(&Version::new(2, 7, 3)),
        "Versions shouldn't match!"
    );
}

#[test]
fn other_test() {
    // todo other tests
}
