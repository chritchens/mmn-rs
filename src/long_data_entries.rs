use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::ToOwned;
use std::ops::Index;
use std::iter::Iterator;
use std::thread;
use crate::result::Result;
use crate::path::tifu_training_data_path;
use crate::raw_data_entry::RawDataEntry;
use crate::long_data_entry::LongDataEntry;

/// `LongDataEntries` represent multiple Long TIFU data entries.
#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LongDataEntries {
    idx: usize,
    len: usize,
    data: Vec<LongDataEntry>,
}

impl LongDataEntries {
    /// `new` creates a new `LongDataEntries`.
    pub fn new() -> LongDataEntries {
        LongDataEntries::default()
    }

    /// `len` returns the `LongDataEntries` number of entries.
    pub fn len(&self) -> usize {
        self.len
    }

    /// `is_empty` returns if the `LongDataEntries` is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `push` pushes a `LongDataEntry` in the `LongDataEntries`.
    pub fn push(&mut self, entry: LongDataEntry) {
        self.len += 1;
        self.data.push(entry);
    }

    /// `pop` pops a `LongDataEntry` from the `LongDataEntries`.
    pub fn pop(&mut self) -> Option<LongDataEntry> {
        self.len -= 1;
        self.data.pop()
    }

    /// `extend_from_slice` extends the `LongDataEntries` with a slice of `LongDataEntry`s.
    pub fn extend_from_slice(&mut self, entries: &[LongDataEntry]) {
        self.len += entries.len();
        self.data.extend_from_slice(entries)
    }

    /// `from_tifu_dataset_file` creates a `LongDataEntries` from `LongDataEntry`s in `TIFU_TRAINING_DATA_PATH`.
    pub fn from_tifu_dataset_file(count: i32) -> Result<LongDataEntries> {
        thread::spawn(move || {
            let path = tifu_training_data_path();
            let file = File::open(&path).map_err(|e| format!("{}", e))?;
            let reader = BufReader::new(file);
            let mut long_data_entries = LongDataEntries::new();

            for (i, line) in reader.lines().enumerate() {
                if i as i32 == count {
                    break;
                }

                let json_raw_data_entry = line.unwrap();
                let raw_data_entry = RawDataEntry::from_json_string(&json_raw_data_entry)?;
                let long_data_entry = LongDataEntry::from_raw(&raw_data_entry);
                long_data_entries.push(long_data_entry);
            }

            Ok(long_data_entries)
        })
        .join()
        .unwrap()
    }

    /// `from_tifu_dataset_file_all` creates a `LongDataEntries` from all the `LongDataEntry`s in `TIFU_TRAINING_DATA_PATH`.
    pub fn from_tifu_dataset_file_all() -> Result<LongDataEntries> {
        LongDataEntries::from_tifu_dataset_file(-1)
    }
}

impl Index<usize> for LongDataEntries {
    type Output = LongDataEntry;

    fn index(&self, idx: usize) -> &LongDataEntry {
        &self.data[idx]
    }
}

impl Iterator for LongDataEntries {
    type Item = LongDataEntry;

    fn next(&mut self) -> Option<LongDataEntry> {
        if self.idx != self.len {
            let item = self.data[self.idx].to_owned();
            self.idx += 1;
            Some(item)
        } else {
            None
        }
    }
}
