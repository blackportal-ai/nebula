//! The module is responsible to provide pods (plain old structs) for datapackage. These are useful for input before validation.
//!
//! The module adds parsing via [serde] use the function [datapackage_meta_from_file_not_validated] to read a datapackage json file.
//! Most users want to use [super::datapackage_meta_from_file] to get a validated datapackage though.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use super::DeltaDataPackageNotValidated;
use super::DeltaDataResourceNotValidated;

/// Reads a json file that contains the datapackage descriptor as json the received data is not checked for validity
///
/// filepath: A path on the filesystem
pub fn datapackage_meta_from_file_not_validated(
    filepath: &Path,
) -> Result<DataPackageNotValidated, std::io::Error> {
    let mut file = File::open(filepath)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let reval: DataPackageNotValidated = serde_json::from_str(buf.as_str())?;
    Ok(reval)
}

/// A mapping for the Data Package json format that is not validated in respect to the schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPackageNotValidated {
    pub resources: Vec<DataResourceNotValidated>,

    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    pub name: Option<String>,

    pub id: Option<String>,

    pub licenses: Vec<DataPackageLicense>,

    pub title: Option<String>,

    pub description: Option<String>,

    pub homepage: Option<String>,

    pub image: Option<String>,

    pub version: Option<String>,

    pub created: Option<String>,

    pub keywords: Option<Vec<String>>,

    pub contributor: Option<Vec<DataPackageContributor>>,

    pub sources: Option<Vec<DataPackageSource>>,

    pub delta: Option<DeltaDataPackageNotValidated>,
}
/// A mapping for the Data Resource json format that is not validated in respect to the schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataResourceNotValidated {
    name: String,

    path: Option<PathSingleOrVec>,

    data: Option<DataStringOrObj>,

    #[serde(rename = "type")]
    type_: Option<String>,

    metadata: Option<String>,

    description: Option<String>,

    format: Option<String>,

    mediatype: Option<String>,

    encoding: Option<String>,

    bytes: Option<u64>,

    hash: Option<String>,

    sources: Option<Vec<DataPackageSource>>,

    delta: Option<DeltaDataResourceNotValidated>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PathSingleOrVec {
    Single(String),
    Vec(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataStringOrObj {
    String(String),

    // todo: how to allow anything like a json value here?
    Object(),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataResourcesData {
    Array(Vec<HashMap<String, String>>),

    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPackageContributor {
    pub title: Option<String>,

    pub given_name: Option<String>,

    pub family_name: Option<String>,

    pub path: Option<String>,

    pub email: Option<String>,

    pub rules: Option<Vec<String>>,

    pub organziation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPackageSource {
    pub title: Option<String>,

    pub path: Option<String>,

    pub email: Option<String>,

    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPackageLicense {
    pub name: String,

    pub path: String,

    pub title: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_data_resource_with_path() {
        let json = r#"
            {
                "name": "Test Resource",
                "path": "path/to/resource"
            }
            "#;

        let data_resource: DataResourceNotValidated = serde_json::from_str(json).unwrap();
        assert_eq!(data_resource.name, "Test Resource".to_string());
    }

    #[test]
    fn test_deserialize_data_resource_with_multiple_paths() {
        let json = r#"
            {
                "name": "Test Resource",
                "path": ["path/to/resource1", "path/to/resource1"]
            }
            "#;

        let data_resource: DataResourceNotValidated = serde_json::from_str(json).unwrap();
        assert_eq!(data_resource.name, "Test Resource".to_string());
    }

    #[test]
    fn test_deserialize_data_resource_with_inline_data_as_string() {
        let json = r#"
            {
                "name": "Binary Inline Resource",
                "data": "ZmRmO2tqYTthamtqa2Y7YWprc2tqc2E7ZGtoZmloO2dpaml1YXF1aGlnanNraGFkZ2pycWF0ODQzMDlpYXJq"
            }
            "#;

        let data_resource: DataResourceNotValidated = serde_json::from_str(json).unwrap();
        assert_eq!(data_resource.name, "Binary Inline Resource".to_string());
    }

    #[test]
    fn test_deserialize_data_resource_with_inline_data_as_json_object() {
        let _json = r#"
            {
                "name": "Inline Data Resource",
                "data": {"a": 22, "b": "a string", "c": [1, 2, 3]}
            }
            "#;

        // todo: reintegrate test
        //let data_resource: DataResourceNotValidated = serde_json::from_str(json).unwrap();
        //assert_eq!(data_resource.name, "Inline Data Resource".to_string());
    }

    #[test]
    fn test_deserialize_data_resource_with_type_given() {
        let json = r#"
            {
                "name": "Test Resource",
                "path": "path/to/resource",
                "type": "table"
            }
            "#;

        let data_resource: DataResourceNotValidated = serde_json::from_str(json).unwrap();
        assert_eq!(data_resource.name, "Test Resource".to_string());
    }
}
