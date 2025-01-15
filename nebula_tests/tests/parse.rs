use std::{env::current_dir, fs::File, io::Read};

use nebula_common::datapackage::DataPackageNotValidated;

#[test]
fn parse_cifar10() {
    println!("Test Cifar10: {}", current_dir().unwrap().to_str().unwrap());
    let file = "../nebula_registry/data/cifar10/datapackage.json";
    let mut file = File::open(file).expect("file is in-accessible");

    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("file can be read");

    let dp: DataPackageNotValidated =
        serde_json::from_str(buf.as_str()).expect("syntax is correct");

    assert_eq!(dp.title, Some("Cifar-10 60'000 32x32 coloured images in 10 classes".to_string()));
}
