use serde::{Serialize, Deserialize};
use crate::raw_data_entry::RawDataEntry;

/// ShortDataEntry is a struct representing an entry in the Short TIFU dataset.
#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct ShortDataEntry {
    pub id: String,
    pub summary: String,
    pub summary_tokenized: Vec<String>,
    pub source: String,
    pub source_tokenized: Vec<String>,
}

impl ShortDataEntry {
    /// `new` creates a new `ShortDataEntry`.
    pub fn new() -> ShortDataEntry {
        ShortDataEntry::default()
    }

    /// `from_raw` creates a `ShortDataEntry` from a `RawDataEntry`.
    pub fn from_raw(rde: &RawDataEntry) -> ShortDataEntry {
        ShortDataEntry {
            id: rde.id.to_owned(),
            summary: rde.trimmed_title.to_owned(),
            summary_tokenized: rde.trimmed_title_tokenized.to_owned(),
            source: rde.selftext_without_tldr.to_owned(),
            source_tokenized: rde.selftext_without_tldr_tokenized.to_owned(),
        }
    }
}
