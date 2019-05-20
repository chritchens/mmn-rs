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
    pub id: String,
    pub selftext_html: Option<String>,
    pub score: u64,
    pub upvote_ratio: f64,
    pub tldr_tokenized: Option<Vec<String>>,
    pub selftext: String,
    pub trimmed_title: String,
    pub selftext_without_tldr_tokenized: Vec<String>,
    pub ups: u64,
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

        if let Some(tt) = v["title_tokenized"].as_array() {
            if !tt.is_empty() {
                let mut res = Vec::new();

                for (i, v) in tt.iter().enumerate() {
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

        if let Some(ttt) = v["trimmed_title_tokenized"].as_array() {
            if !ttt.is_empty() {
                let mut res = Vec::new();

                for (i, v) in ttt.iter().enumerate() {
                    if let Some(x) = v.as_str().map(ToOwned::to_owned) {
                        res.push(x);
                    } else {
                        return Err(format!("invalid trimmed_title_tokenized field element at index: {}", i));
                    }
                }

                entry.trimmed_title_tokenized = res;
            }
        } else if !v["trimmed_title_tokenized"].is_array() && !v["selftext_without_tldr_tokenized"].is_null() {
            return Err("invalid trimmed_title_tokenized field".to_string());
        }

        if let Some(swtt) = v["selftext_without_tldr_tokenized"].as_array() {
            if !swtt.is_empty() {
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
            if !tt.is_empty() {
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

        entry.selftext_html = v["selftext_html"].as_str().map(ToOwned::to_owned);

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

#[cfg(test)]
mod test {
    use super::DataEntry;

    const valid_entry: &str = r#"{"title_tokenized": ["tifu", "by", "forgetting", "to", "pull", "my", "underwear", "down", "before", "i", "pooped"], "permalink": "/r/tifu/comments/1ghd5r/tifu_by_forgetting_to_pull_my_underwear_down/", "title": "TIFU by forgetting to pull my underwear down before I pooped.", "url": "https://www.reddit.com/r/tifu/comments/1ghd5r/tifu_by_forgetting_to_pull_my_underwear_down/", "num_comments": 13, "tldr": null, "created_utc": 1371426179.0, "trimmed_title_tokenized": ["forgetting", "to", "pull", "my", "underwear", "down", "before", "i", "pooped"], "id": "1ghd5r", "selftext_html": "<!-- SC_OFF --><div class=\"md\"><p>I was on Skype on my tablet as I went to the toilet IMing a friend. I don&#39;t multitask very well, so I forgot one of the most important things to do before pooping. I think the best part was when I realised and told my mate who just freaked out because I was talking to him on the John!</p>\n</div><!-- SC_ON -->", "score": 50, "upvote_ratio": 0.77, "tldr_tokenized": null, "selftext": "I was on Skype on my tablet as I went to the toilet IMing a friend. I don't multitask very well, so I forgot one of the most important things to do before pooping. I think the best part was when I realised and told my mate who just freaked out because I was talking to him on the John!", "trimmed_title": "forgetting to pull my underwear down before i pooped.", "selftext_without_tldr_tokenized": ["i", "was", "on", "skype", "on", "my", "tablet", "as", "i", "went", "to", "the", "toilet", "iming", "a", "friend", "i", "do", "n't", "multitask", "very", "well", "so", "i", "forgot", "one", "of", "the", "most", "important", "things", "to", "do", "before", "pooping", "i", "think", "the", "best", "part", "was", "when", "i", "realised", "and", "told", "my", "mate", "who", "just", "freaked", "out", "because", "i", "was", "talking", "to", "him", "on", "the", "john"], "ups": 50, "selftext_without_tldr": "i was on skype on my tablet as i went to the toilet iming a friend. i don't multitask very well, so i forgot one of the most important things to do before pooping. i think the best part was when i realised and told my mate who just freaked out because i was talking to him on the john!"}"#;

    fn valid_entry_into_bytes() -> Vec<u8> {
       String::from(valid_entry).into_bytes()
    }

    #[test]
    fn test_serialize_entry() {
        let res = DataEntry::from_json_string(valid_entry);
        assert!(res.is_ok());
        let data_entry_0 = res.unwrap();
        let res = data_entry_0.to_json_string();
        assert!(res.is_ok());
        let json_string_data_entry_0 = res.unwrap();
        assert_eq!(data_entry_0, DataEntry::from_json_string(&json_string_data_entry_0).unwrap());

        let res = serde_json::from_str(valid_entry);
        assert!(res.is_ok());
        let json_value = res.unwrap();

        let res = DataEntry::from_json_value(&json_value);
        assert!(res.is_ok());
        let data_entry_1 = res.unwrap();
        let res = data_entry_1.to_json_string();
        assert!(res.is_ok());
        let json_string_data_entry_1 = res.unwrap();
        assert_eq!(data_entry_1, DataEntry::from_json_string(&json_string_data_entry_1).unwrap());

        let valid_entry_bytes = valid_entry_into_bytes();
        let res = DataEntry::from_json_bytes(&valid_entry_bytes);
        assert!(res.is_ok());
        let data_entry_2 = res.unwrap();
        let res = data_entry_2.to_json_bytes();
        assert!(res.is_ok());
        let json_bytes_data_entry_2 = res.unwrap();
        assert_eq!(data_entry_2, DataEntry::from_json_bytes(&json_bytes_data_entry_2).unwrap());

        assert_eq!(data_entry_0, data_entry_1);
        assert_eq!(data_entry_1, data_entry_2);
    }

    #[test]
    fn test_missing_fields() {
        let res = serde_json::from_str(valid_entry);
        assert!(res.is_ok());
        let json_value: serde_json::Value = res.unwrap();

        let mut missing_title_obj = json_value.clone().as_object().unwrap().to_owned();
        missing_title_obj.remove("title");

        let missing_title_value: serde_json::Value = missing_title_obj.into();
        let res = DataEntry::from_json_value(&missing_title_value);
        assert!(res.is_err());

        let mut missing_ups_obj = json_value.clone().as_object().unwrap().to_owned();
        missing_ups_obj.remove("ups");

        let missing_ups_value: serde_json::Value = missing_ups_obj.into();
        let res = DataEntry::from_json_value(&missing_ups_value);
        assert!(res.is_err());

        let mut missing_tldr_tokenized_obj = json_value.clone().as_object().unwrap().to_owned();
        missing_tldr_tokenized_obj.remove("tldr_tokenized");

        let missing_tldr_tokenized_value: serde_json::Value = missing_tldr_tokenized_obj.into();
        let res = DataEntry::from_json_value(&missing_tldr_tokenized_value);
        assert!(res.is_ok());
    }

    #[test]
    fn test_null_fields() {
        let res = serde_json::from_str(valid_entry);
        assert!(res.is_ok());
        let json_value: serde_json::Value = res.unwrap();

        let mut null_selftext_obj = json_value.clone().as_object().unwrap().to_owned();
        null_selftext_obj["selftext"] = serde_json::json!(null);

        let null_selftext_value: serde_json::Value = null_selftext_obj.into();
        let res = DataEntry::from_json_value(&null_selftext_value);
        assert!(res.is_err());

        let mut null_score_obj = json_value.clone().as_object().unwrap().to_owned();
        null_score_obj["score"] = serde_json::json!(null);

        let null_score_value: serde_json::Value = null_score_obj.into();
        let res = DataEntry::from_json_value(&null_score_value);
        assert!(res.is_err());

        let mut null_tldr_obj = json_value.clone().as_object().unwrap().to_owned();
        null_tldr_obj["tldr"] = serde_json::json!(null);

        let null_tldr_value: serde_json::Value = null_tldr_obj.into();
        let res = DataEntry::from_json_value(&null_tldr_value);
        assert!(res.is_ok());
    }

    #[test]
    fn test_num_fields() {
        let res = serde_json::from_str(valid_entry);
        assert!(res.is_ok());
        let json_value: serde_json::Value = res.unwrap();

        let mut f64_num_comments_obj = json_value.clone().as_object().unwrap().to_owned();
        f64_num_comments_obj["num_comments"] = serde_json::json!(0.1234);

        let f64_num_comments_value: serde_json::Value = f64_num_comments_obj.into();
        let res = DataEntry::from_json_value(&f64_num_comments_value);
        assert!(res.is_err());

        let mut f64_score_obj = json_value.clone().as_object().unwrap().to_owned();
        f64_score_obj["score"] = serde_json::json!(0.1234);

        let f64_score_value: serde_json::Value = f64_score_obj.into();
        let res = DataEntry::from_json_value(&f64_score_value);
        assert!(res.is_err());

        let mut f64_ups_obj = json_value.clone().as_object().unwrap().to_owned();
        f64_ups_obj["ups"] = serde_json::json!(0.1234);

        let f64_ups_value: serde_json::Value = f64_ups_obj.into();
        let res = DataEntry::from_json_value(&f64_ups_value);
        assert!(res.is_err());

        let mut u64_created_utc_obj = json_value.clone().as_object().unwrap().to_owned();
        u64_created_utc_obj["created_utc"] = serde_json::json!(1234);

        let u64_created_utc_value: serde_json::Value = u64_created_utc_obj.into();
        let res = DataEntry::from_json_value(&u64_created_utc_value);
        assert!(res.is_ok());
    }
}
