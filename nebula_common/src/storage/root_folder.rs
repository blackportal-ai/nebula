use std::{collections::HashMap, fs::create_dir_all, ops::Deref, path::PathBuf, str::FromStr};

use color_eyre::eyre::{Report, eyre};
use tracing::info;
use uuid::Uuid;

use crate::{
    datapackage::{DataPackage, datapackage_meta_from_file},
    model::{FieldSettings, FilterSettings, PagationSettings, SortSettings},
};

use async_trait::async_trait;

use super::MetaDataSource;

/// Reads datapackage.json files from the filesystem
#[derive(Debug)]
pub struct RootFolderSource {
    path: PathBuf,

    buf: HashMap<PathBuf, (Uuid, DataPackage)>,
}

fn get_datapackage_file_candidates_from_folder(
    path: &PathBuf,
    candidates: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // check folders for datapackage.json:
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let mut path = entry.path();
            if path.is_dir() {
                // 1. check for datapackage.json
                path.push("datapackage.json");
                if path.is_file() {
                    candidates.push(path);
                }
            }
        }
    }

    Ok(())
}

impl RootFolderSource {
    pub fn new_from_folder(path: PathBuf) -> Self {
        let mut reval = RootFolderSource { path, buf: HashMap::new() };
        reval.sync_all().unwrap();
        reval
    }

    fn sync_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut candidates = vec![];
        get_datapackage_file_candidates_from_folder(&self.path, &mut candidates)?;

        for candidate in candidates {
            match self.sync_file(candidate) {
                Ok(_) => {}
                Err(_) => {
                    // todo error reporting
                }
            }
        }
        Ok(())
    }

    fn sync_file(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let dp = datapackage_meta_from_file(&file_path)?;
        let id = if let Some((id, _)) = self.buf.get(&file_path) { *id } else { Uuid::new_v4() };
        self.buf.insert(file_path, (id, dp));
        Ok(())
    }
}

#[async_trait]
impl MetaDataSource for RootFolderSource {
    async fn list_packages(
        &self,
        _sort: SortSettings,
        _filter: FilterSettings,
        _pagation: PagationSettings,
        _fields: FieldSettings,
    ) -> Vec<DataPackage> {
        self.buf.values().map(|(_, v)| v).cloned().collect()
    }

    async fn get_package(&self, query: &str, _filter: FilterSettings) -> Option<DataPackage> {
        info!("get_package");
        self.buf
            .values()
            .for_each(|el| info!("id: {}, name='{}'", el.0, el.1.name.clone().unwrap()));

        self.buf
            .values()
            .filter_map(|(_, v)| {
                if v.name.clone().map_or(false, |el| el.contains(query)) { Some(v) } else { None }
            })
            .nth(0)
            .cloned()
    }

    async fn search_package(
        &self,
        _search_query: &str,
        _sort: SortSettings,
        _filter: FilterSettings,
        _pagation: PagationSettings,
    ) -> Vec<DataPackage> {
        todo!()
    }

    async fn put_package_metadata(&mut self, package: &DataPackage) -> Result<(), Report> {
        // todo: error if version or name not given

        // some sanity checks:
        let name = if let Some(name) = package.name.as_ref() {
            name
        } else {
            return Err(eyre!("package missing name"));
        };

        let version = if let Some(version) = package.name.as_ref() {
            version
        } else {
            return Err(eyre!("Package missing version"));
        };

        let id: Uuid = if let Some(id) = package.id.as_ref() {
            match Uuid::from_str(id) {
                Ok(id) => id,
                Err(_) => return Err(eyre!("Malformed id")),
            }
        } else {
            return Err(eyre!("Package missing id"));
        };

        // ensure folder is there
        let folder = self.path.join(name).join(version);
        create_dir_all(folder.clone()).unwrap();
        let json = serde_json::ser::to_string_pretty(package.deref()).unwrap();
        let dp_path = folder.join("datapackage.json");
        std::fs::write(&dp_path, json).unwrap();

        // save to local buffer:
        self.buf.insert(dp_path, (id, package.clone()));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::datapackage::{DataPackageNotValidated, DataResourceNotValidated, ValidateData};

    #[tokio::test]
    #[should_panic]
    pub async fn test_put_missing_version() {
        let mut rf = RootFolderSource::new_from_folder(PathBuf::from_str("tmp").unwrap());
        let res = DataResourceNotValidated { name: "iris.csv".into(), ..Default::default() };
        let package = DataPackageNotValidated {
            resources: vec![res],
            name: Some("iris".into()),
            ..Default::default()
        };

        let dp = package.validate().unwrap();
        rf.put_package_metadata(&dp).await.expect("version is missing, so should panic");
    }

    #[tokio::test]
    pub async fn test_put_valid() {
        let mut rf = RootFolderSource::new_from_folder(PathBuf::from_str("tmp").unwrap());
        let res = DataResourceNotValidated { name: "iris.csv".into(), ..Default::default() };
        let package = DataPackageNotValidated {
            resources: vec![res],
            name: Some("iris".into()),
            version: Some("0.1.0".into()),
            ..Default::default()
        };

        let dp = package.validate().unwrap();
        rf.put_package_metadata(&dp).await.unwrap();

        assert!(std::fs::exists(PathBuf::from_str("./tmp/datapackage.json").unwrap()).is_ok())
    }
}
