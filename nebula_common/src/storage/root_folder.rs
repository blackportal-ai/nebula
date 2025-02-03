use std::{collections::HashMap, path::PathBuf};

use uuid::Uuid;

use crate::{
    datapackage::{DataPackage, datapackage_meta_from_file},
    model::{FieldSettings, FilterSettings, PagationSettings, SortSettings},
};

use super::MetaDataSource;

/// Reads datapackage.json files from the filesystem
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

    async fn get_package(
        &self,
        query: &str,
        _filter: FilterSettings,
    ) -> Option<crate::datapackage::DataPackage> {
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
}
