use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Serialize, Default)]
pub struct CardSearch {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_extras: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_multilingual: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_variations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

mod uri_serde {
    use http::Uri;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uri>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => s.parse::<Uri>().map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
    pub fn serialize<S>(uri: &Option<Uri>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match uri {
            Some(uri) => serializer.serialize_some(&uri.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ReturnList<T> {
    pub object: String,
    pub data: Vec<T>,
    pub has_more: bool,
    #[serde(with = "uri_serde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub next_page: Option<http::Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cards: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<Vec<String>>,
}
