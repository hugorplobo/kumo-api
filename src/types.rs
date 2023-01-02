use std::{str::FromStr, marker::PhantomData};

use serde::{Deserialize, Deserializer, de::{Visitor, self}, Serialize};
use void::Void;

use crate::database::Database;

pub struct AppState {
    pub database: Database
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub username: String
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub query_id: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub user: User,
    pub auth_date: String,
    pub hash: String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

impl FromStr for User {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            serde_json::from_str(s).unwrap()
        )
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            Ok(FromStr::from_str(v).unwrap())
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}