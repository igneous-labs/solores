use std::{
    collections::HashSet,
    fmt,
    fs::{File, OpenOptions},
    hash::Hash,
    marker::PhantomData,
    path::Path,
    str::FromStr,
};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use void::Void;

pub fn open_file_create_overwrite<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
}

/// Copied from https://serde.rs/string-or-struct.html
pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

pub struct UniqueByReportDupsResult<'a, T> {
    pub unique: Vec<&'a T>,
    pub duplicates: Vec<&'a T>,
}

/// Preserves order of vals
pub fn unique_by_report_dups<'a, I, T, V, F>(vals: I, mut f: F) -> UniqueByReportDupsResult<'a, T>
where
    I: Iterator<Item = &'a T>,
    V: Eq + Hash,
    F: FnMut(&T) -> V,
{
    let mut hashes = HashSet::new();
    let mut unique = Vec::new();
    let mut duplicates = Vec::new();
    for val in vals {
        let hash = f(val);
        if hashes.contains(&hash) {
            duplicates.push(val);
        } else {
            hashes.insert(hash);
            unique.push(val);
        }
    }
    UniqueByReportDupsResult { unique, duplicates }
}
