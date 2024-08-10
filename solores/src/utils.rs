use std::{
    collections::HashSet,
    fmt,
    fs::{File, OpenOptions},
    hash::Hash,
    marker::PhantomData,
    path::Path,
    str::FromStr,
};

use heck::ToPascalCase;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use void::Void;

pub const PUBKEY_TOKEN: &str = "Pubkey";

pub fn primitive_or_pubkey_to_token(s: &str) -> String {
    match s {
        "publicKey" => PUBKEY_TOKEN.to_owned(),
        "string" => s.to_pascal_case(),
        "bytes" => {
            #[cfg(feature = "bytes_to_u8")]
            {
                "u8".to_owned()
            }
            #[cfg(not(feature = "bytes_to_u8"))]
            {
                "bytes".to_owned()
            }
        }
        _ => s.to_owned(),
    }
}

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

pub fn conditional_pascal_case(s: &str) -> String {
    // Only apply PascalCase if the string does not start with an uppercase letter.
    if s.chars().next().map_or(false, |c| c.is_uppercase()) {
        s.to_string()
    } else {
        s.to_pascal_case()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "bytes_to_u8")]
    fn test_bytes_to_u8_feature_enabled() {
        let result = primitive_or_pubkey_to_token("bytes");
        assert_eq!(result, "u8");

        let result = primitive_or_pubkey_to_token("publicKey");
        assert_eq!(result, PUBKEY_TOKEN.to_owned());

        let result = primitive_or_pubkey_to_token("string");
        assert_eq!(result, "String");
    }

    #[test]
    #[cfg(not(feature = "bytes_to_u8"))]
    fn test_bytes_to_u8_feature_disabled() {
        let result = primitive_or_pubkey_to_token("bytes");
        assert_eq!(result, "bytes");

        let result = primitive_or_pubkey_to_token("publicKey");
        assert_eq!(result, PUBKEY_TOKEN.to_owned());

        let result = primitive_or_pubkey_to_token("string");
        assert_eq!(result, "String");
    }

    #[test]
    fn test_already_uppercase() {
        let input = "I80F48";
        let expected = "I80F48";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_lowercase_single_word() {
        let input = "pool";
        let expected = "Pool";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_mixed_case_string() {
        let input = "exampleString";
        let expected = "ExampleString";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_already_pascal_case() {
        let input = "PascalCase";
        let expected = "PascalCase";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_multiple_words() {
        let input = "multiple words";
        let expected = "MultipleWords";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_numeric_start() {
        let input = "123abc";
        let expected = "123abc";
        assert_eq!(conditional_pascal_case(input), expected);
    }

    #[test]
    fn test_uppercase_first_letter() {
        let input = "Uppercase";
        let expected = "Uppercase";
        assert_eq!(conditional_pascal_case(input), expected);
    }
}
