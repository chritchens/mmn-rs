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

#[cfg(test)]
mod test {
    use super::LongDataEntry;
    use crate::raw_data_entries::RawDataEntries;

    #[test]
    fn test_long_data_entry_from_raw() {
        let count = 10;
        let rds = RawDataEntries::from_tifu_dataset_file(count).unwrap();
        for rd in rds {
            let ld = LongDataEntry::from_raw(&rd);
            assert_eq!(&ld.id, &rd.id);
            assert_eq!(&ld.summary, &rd.tldr);
            assert_eq!(&ld.summary_tokenized, &rd.tldr_tokenized);
            assert_eq!(&ld.source, &rd.selftext_without_tldr);
            assert_eq!(&ld.source_tokenized, &rd.selftext_without_tldr_tokenized);
        }
    }
}
