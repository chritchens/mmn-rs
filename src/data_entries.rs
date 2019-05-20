use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::borrow::ToOwned;
use std::ops::Index;
use std::iter::Iterator;
use std::thread;
use crate::result::Result;
use crate::path::tifu_training_data_path;
use crate::data_entry::DataEntry;

/// `DataEntries` represent multiple data entries.
#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct DataEntries {
    idx: usize,
    len: usize,
    data: Vec<DataEntry>,
}

impl DataEntries {
    /// `new` creates a new `DataEntries`.
    pub fn new() -> DataEntries {
        DataEntries::default()
    }

    /// `len` returns the `DataEntries` number of entries.
    pub fn len(&self) -> usize {
        self.len
    }

    /// `is_empty` returns if the `DataEntries` is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `push` pushes a `DataEntry` in the `DataEntries`.
    pub fn push(&mut self, entry: DataEntry) {
        self.len += 1;
        self.data.push(entry.to_owned());
    }

    /// `pop` pops a `DataEntry` from the `DataEntries`.
    pub fn pop(&mut self) -> Option<DataEntry> {
        self.len -= 1;
        self.data.pop()
    }

    /// `extend_from_slice` extends the `DataEntries` with a slice of `DataEntry`s.
    pub fn extend_from_slice(&mut self, entries: &[DataEntry]) {
        self.len += entries.len();
        self.data.extend_from_slice(entries)
    }

    /// `from_tifu_training_data` creates a `DataEntries` from the `DataEntry`s in `TIFU_TRAINING_DATA_PATH`.
    pub fn from_tifu_training_data() -> Result<DataEntries> {
        thread::spawn(move || {
            let path = tifu_training_data_path();
            let mut file = File::open(&path).map_err(|e| format!("{}", e))?;
            let mut text = String::new();
            file.read_to_string(&mut text).map_err(|e| format!("{}", e))?;
            let lines = text.lines();

            let mut data_entries = DataEntries::new();

            for line in lines {
                data_entries.push(DataEntry::from_json_string(&line)?);
            }

            Ok(data_entries)
        })
        .join()
        .unwrap()
    }
}

impl Index<usize> for DataEntries {
    type Output = DataEntry;

    fn index(&self, idx: usize) -> &DataEntry {
        &self.data[idx]
    }
}

impl Iterator for DataEntries {
    type Item = DataEntry;

    fn next(&mut self) -> Option<DataEntry> {
        if self.idx != self.len -1 {
            self.idx += 1;
            Some(self.data[self.idx].to_owned())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::DataEntries;
    use crate::data_entry::DataEntry;

    #[test]
    fn test_data_entries_modifiers() {
        let d  = DataEntry::new();
        let d1 = d.clone();
        let d2 = d.clone();
        let d3 = d.clone();

        let mut ds = DataEntries::new();
        assert_eq!(ds.len(), 0);
        assert!(ds.is_empty());

        ds.push(d1);
        assert_eq!(ds.len(), 1);
        assert!(!ds.is_empty());

        let dx_opt = ds.pop();
        assert!(dx_opt.is_some());

        let dx = dx_opt.unwrap();
        assert_eq!(d, dx);
        assert_eq!(ds.len(), 0);
        assert!(ds.is_empty());

        ds.push(d2);
        ds.push(d3);
        assert_eq!(ds.len(), 2);
        assert!(!ds.is_empty());

        let dy_opt = ds.pop();
        assert!(dy_opt.is_some());
        let dz_opt = ds.pop();
        assert!(dz_opt.is_some());

        let dy = dy_opt.unwrap();
        let dz = dz_opt.unwrap();
        assert_eq!(dy, d);
        assert_eq!(dz, d);
        assert_eq!(ds.len(), 0);
        assert!(ds.is_empty());
    }

    #[test]
    fn test_data_entries_tifu_datasets() {}
}
