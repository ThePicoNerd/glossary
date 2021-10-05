use std::{collections::HashMap, error::Error, fs::File, path::Path};

use serde::Serialize;

use crate::definition::Definitions;

type EntryMap = HashMap<String, Definitions>;

#[derive(Debug, Serialize)]
pub struct Entry {
    pub term: String,
    pub definitions: Definitions,
}

impl From<(String, Definitions)> for Entry {
    fn from((term, definitions): (String, Definitions)) -> Self {
        Self { term, definitions }
    }
}

#[derive(Debug)]
pub struct Dictionary {
    pub name: String,
    pub terms: Vec<Entry>,
}

impl Dictionary {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let map: EntryMap = serde_yaml::from_reader(file)?;

        let terms: Vec<Entry> = map.into_iter().map(|t| t.into()).collect();

        Ok(Self {
            name: path.file_stem().unwrap().to_string_lossy().to_string(),
            terms,
        })
    }
}
