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

#[cfg(test)]
mod test {
    use super::LongDataEntries;
    use crate::long_data_entry::LongDataEntry;
    use std::iter::Iterator;

    #[test]
    fn test_long_data_entries_accessors() {
        let mut d1 = LongDataEntry::new();
        d1.summary = Some("d1".to_string());
        let mut d2 = LongDataEntry::new();
        d2.summary = Some("d2".to_string());
        let mut d3 = LongDataEntry::new();
        d3.summary = Some("d3".to_string());
        
        let mut ds = LongDataEntries::new();
        assert_eq!(ds.len(), 0);
        assert!(ds.is_empty());

        ds.push(d1.clone());
        assert_eq!(ds.len(), 1);
        assert!(!ds.is_empty());
        assert_eq!(ds[0], d1);

        ds.push(d2.clone());
        assert_eq!(ds.len(), 2);
        assert!(!ds.is_empty());
        assert_eq!(ds[0], d1);
        assert_eq!(ds[1], d2);

        ds.push(d3.clone());
        assert_eq!(ds.len(), 3);
        assert!(!ds.is_empty());
        assert_eq!(ds[0], d1);
        assert_eq!(ds[1], d2);
        assert_eq!(ds[2], d3);

        for (i, v) in ds.clone().enumerate() {
            assert_eq!(v, ds[i]);
        }
    }

    #[test]
    fn test_long_data_entries_modifiers() {
        let d  = LongDataEntry::new();
        let d1 = d.clone();
        let d2 = d.clone();
        let d3 = d.clone();

        let mut ds = LongDataEntries::new();
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
    fn test_long_data_entries_from_tifu_dataset_file() {
        let count_1 = 0;
        let count_2 = 10;
        let count_3 = 20;

        let res = LongDataEntries::from_tifu_dataset_file(count_1);
        assert!(res.is_ok());

        let ds_1 = res.unwrap();
        assert_eq!(ds_1.len(), count_1 as usize);

        let res = LongDataEntries::from_tifu_dataset_file(count_2);
        assert!(res.is_ok());

        let ds_2 = res.unwrap();
        assert_eq!(ds_2.len(), count_2 as usize);

        let res = LongDataEntries::from_tifu_dataset_file(count_3);
        assert!(res.is_ok());

        let ds_3 = res.unwrap();
        assert_eq!(ds_3.len(), count_3 as usize);
    }
}
