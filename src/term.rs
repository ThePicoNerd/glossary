use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    fs::File,
    marker::PhantomData,
    path::Path,
};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Deserialize)]
pub struct Definitions(#[serde(deserialize_with = "string_or_seq_string")] pub Vec<String>);

impl Display for Definitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("; "))
    }
}

impl Serialize for Definitions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

type EntryMap = HashMap<String, Definitions>;

#[derive(Debug, Serialize)]
pub struct Entry {
    pub term: String,
    pub definitions: Definitions,
}

impl From<(String, Definitions)> for Entry {
    fn from((term, definitions): (String, Definitions)) -> Self {
        Self {
            term,
            definitions,
        }
    }
}

fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or sequence of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
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
