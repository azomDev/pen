use home;
use pen::utils;

#[test]
fn get_version_path() {
    let home_dir = home::home_dir().expect("Failed to get home directory"); // so it works for anyone testing, this is specific to get_version_path
    let expected_path = home_dir.join(".pen/python_versions/python_3.11.9");
    let version_path = utils::get_version_path("3.11.9");
    assert_eq!(version_path, expected_path);
}


#[test]
fn other_test() {
    // todo other tests
}
