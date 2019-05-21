use serde::{Serialize, Deserialize};
use crate::raw_data_entry::RawDataEntry;

/// LongDataEntry is a struct representing an entry in the Long TIFU dataset.
#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LongDataEntry {
    pub id: String,
    pub summary: Option<String>,
    pub summary_tokenized: Option<Vec<String>>,
    pub source: String,
    pub source_tokenized: Vec<String>,
}

impl LongDataEntry {
    /// `new` creates a new `LongDataEntry`.
    pub fn new() -> LongDataEntry {
        LongDataEntry::default()
    }

    /// `from_raw` creates a `LongDataEntry` from a `RawDataEntry`.
    pub fn from_raw(rde: &RawDataEntry) -> LongDataEntry {
        LongDataEntry {
            id: rde.id.to_owned(),
            summary: rde.tldr.to_owned(),
            summary_tokenized: rde.tldr_tokenized.to_owned(),
            source: rde.selftext_without_tldr.to_owned(),
            source_tokenized: rde.selftext_without_tldr_tokenized.to_owned(),
        }
    }
}

