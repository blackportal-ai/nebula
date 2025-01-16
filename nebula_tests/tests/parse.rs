use std::path::Path;

use nebula_common::datapackage::datapackage_meta_from_file_not_validated;

#[test]
fn test_parse_cifar10() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "../nebula_registry/data/cifar10/datapackage.json";
    let dp = datapackage_meta_from_file_not_validated(Path::new(filepath))?;

    assert_eq!(dp.title, Some("Cifar-10 60'000 32x32 coloured images in 10 classes".to_string()));
    assert!(dp.delta.is_some());
    let delta = dp.delta.unwrap();
    assert_eq!(delta.category, "classification");

    // todo: more tests
    Ok(())
}

#[test]
fn test_parse_iris() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "../nebula_registry/data/iris/datapackage.json";
    let dp = datapackage_meta_from_file_not_validated(Path::new(filepath))?;

    assert_eq!(dp.title, Some("Iris flower dataset".to_string()));
    assert!(dp.delta.is_some());
    let delta = dp.delta.unwrap();
    assert_eq!(delta.classes.unwrap(), 3);
    Ok(())
}
