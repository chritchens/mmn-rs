use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use crate::result::Result;

/// DataEntry is a struct representing an entry in the json training data entry.
#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct DataEntry {
    pub title_tokenized: Vec<String>,
    pub permalink: String,
    pub title: String,
    pub url: String,
    pub num_comments: u64,
    pub tldr: Option<String>,
    pub created_utc: f64,
    pub trimmed_title_tokenized: Vec<String>,
    pub ups: u64,
    pub selftext_html: Option<String>,
    pub score: u64,
    pub upvote_ratio: f64,
    pub tldr_tokenized: Option<Vec<String>>,
    pub selftext: String,
    pub trimmed_title: String,
    pub selftext_without_tldr_tokenized: Vec<String>,
    pub id: String,
    pub selftext_without_tldr: String,
}

impl DataEntry {
    /// `new` creates a new `DataEntry`.
    pub fn new() -> DataEntry {
        DataEntry::default()
    }

    /// `from_json_value` converts a `serde_json::Value` into a `DataEntry`.
    pub fn from_json_value(v: &Value) -> Result<DataEntry> {
        let mut entry = DataEntry::new();

        if let Some(ttd) = v["title_tokenized"].as_array() {
            if ttd.is_empty() {
                let mut res = Vec::new();

                for (i, v) in ttd.iter().enumerate() {
                    if let Some(x) = v.as_str().map(ToOwned::to_owned) {
                        res.push(x);
                    } else {
                        return Err(format!("invalid title_tokenized field element at index: {}", i));
                    }
                }

                entry.title_tokenized = res;
            }
        } else if !v["title_tokenized"].is_array() && !v["selftext_without_tldr_tokenized"].is_null() {
            return Err("invalid title_tokenized field".to_string());
        }

        if let Some(swtt) = v["selftext_without_tldr_tokenized"].as_array() {
            if swtt.is_empty() {
                let mut res = Vec::new();

                for (i, v) in swtt.iter().enumerate() {
                    if let Some(x) = v.as_str().map(ToOwned::to_owned) {
                        res.push(x);
                    } else {
                        return Err(format!("invalid selftext_without_tldr_tokenized field element at index: {}", i));
                    }
                }

                entry.selftext_without_tldr_tokenized = res;
            }
        } else if !v["selftext_without_tldr_tokenized"].is_array() && !v["selftext_without_tldr_tokenized"].is_null() {
            return Err("invalid selftext_without_tldr_tokenized field".to_string());
        }

        if let Some(tt) = v["tldr_tokenized"].as_array() {
            if tt.is_empty() {
                let mut res = Vec::new();

                for (i, v) in tt.iter().enumerate() {
                    if let Some(x) = v.as_str().map(ToOwned::to_owned) {
                        res.push(x);
                    } else {
                        return Err(format!("invalid tldr_tokenized field element at index: {}", i));
                    }
                }

                entry.tldr_tokenized = Some(res);
            }
        } else if !v["tldr_tokenized"].is_array() && !v["tldr_tokenized"].is_null() {
            return Err("invalid tldr_tokenized field".to_string());
        }

        if let Some(p) = v["permalink"].as_str().map(ToOwned::to_owned) {
            entry.permalink = p;
        } else {
            return Err("invalid permalink field".to_string());
        }

        if let Some(t) = v["title"].as_str().map(ToOwned::to_owned) {
            entry.title = t;
        } else {
            return Err("invalid title field".to_string());
        }

        if let Some(u) = v["url"].as_str().map(ToOwned::to_owned) {
            entry.url = u;
        } else {
            return Err("invalid url field".to_string());
        }

        if let Some(st) = v["selftext"].as_str().map(ToOwned::to_owned) {
            entry.selftext = st;
        } else {
            return Err("invalid selftext field".to_string());
        }

        if let Some(tt) = v["trimmed_title"].as_str().map(ToOwned::to_owned) {
            entry.trimmed_title = tt;
        } else {
            return Err("invalid trimmed_title field".to_string());
        }

        if let Some(swt) = v["selftext_without_tldr"].as_str().map(ToOwned::to_owned) {
            entry.selftext_without_tldr = swt;
        } else {
            return Err("invalid selftext_without_tldr field".to_string());
        }

        if let Some(i) = v["id"].as_str().map(ToOwned::to_owned) {
            entry.id = i;
        } else {
            return Err("invalid id field".to_string());
        }

        if let Some(nc) = v["num_comments"].as_u64() {
            entry.num_comments = nc;
        } else {
            return Err("invalid num_comments field".to_string());
        }

        if let Some(u) = v["ups"].as_u64() {
            entry.ups = u;
        } else {
            return Err("invalid ups field".to_string());
        }

        if let Some(s) = v["score"].as_u64() {
            entry.score = s;
        } else {
            return Err("invalid score field".to_string());
        }

        if let Some(cu) = v["created_utc"].as_f64() {
            entry.created_utc = cu;
        } else {
            return Err("invalid created_utc field".to_string());
        }

        if let Some(ur) = v["upvote_ratio"].as_f64() {
            entry.upvote_ratio = ur;
        } else {
            return Err("invalid upvote_ratio field".to_string());
        }

        entry.tldr = v["tldr"].as_str().map(ToOwned::to_owned);

        entry.selftext_html = v["tldr"].as_str().map(ToOwned::to_owned);

        Ok(entry)
    }

    /// `from_json_string` converts a json `str` to a `DataEntry`.
    pub fn from_json_string(s: &str) -> Result<DataEntry> {
        let value: Value = serde_json::from_str(s)
            .map_err(|e| format!("{}", e))?;
        DataEntry::from_json_value(&value)
    }

    /// `to_json_string` converts the `DataEntry` to a `String`.
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
            .map_err(|e| format!("{}", e))
    }

    /// `from_json_bytes` converts a `&[u8]` to `DataEntry`.
    pub fn from_json_bytes(b: &[u8]) -> Result<DataEntry> {
        let value: Value = serde_json::from_slice(b)
            .map_err(|e| format!("{}", e))?;
        DataEntry::from_json_value(&value)
    }

    /// `to_json_bytes` converts the `DataEntry` to a `Vec<u8>`.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| format!("{}", e))
    }
}
