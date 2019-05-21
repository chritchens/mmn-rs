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
use crate::short_data_entry::ShortDataEntry;

/// `ShortDataEntries` represent multiple Short TIFU data entries.
#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct ShortDataEntries {
    idx: usize,
    len: usize,
    data: Vec<ShortDataEntry>,
}

impl ShortDataEntries {
    /// `new` creates a new `ShortDataEntries`.
    pub fn new() -> ShortDataEntries {
        ShortDataEntries::default()
    }

    /// `len` returns the `ShortDataEntries` number of entries.
    pub fn len(&self) -> usize {
        self.len
    }

    /// `is_empty` returns if the `ShortDataEntries` is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `push` pushes a `ShortDataEntry` in the `ShortDataEntries`.
    pub fn push(&mut self, entry: ShortDataEntry) {
        self.len += 1;
        self.data.push(entry);
    }

    /// `pop` pops a `ShortDataEntry` from the `ShortDataEntries`.
    pub fn pop(&mut self) -> Option<ShortDataEntry> {
        self.len -= 1;
        self.data.pop()
    }

    /// `extend_from_slice` extends the `ShortDataEntries` with a slice of `ShortDataEntry`s.
    pub fn extend_from_slice(&mut self, entries: &[ShortDataEntry]) {
        self.len += entries.len();
        self.data.extend_from_slice(entries)
    }

    /// `from_tifu_dataset_file` creates a `ShortDataEntries` from `ShortDataEntry`s in `TIFU_TRAINING_DATA_PATH`.
    pub fn from_tifu_dataset_file(count: i32) -> Result<ShortDataEntries> {
        thread::spawn(move || {
            let path = tifu_training_data_path();
            let file = File::open(&path).map_err(|e| format!("{}", e))?;
            let reader = BufReader::new(file);
            let mut short_data_entries = ShortDataEntries::new();

            for (i, line) in reader.lines().enumerate() {
                if i as i32 == count {
                    break;
                }

                let json_raw_data_entry = line.unwrap();
                let raw_data_entry = RawDataEntry::from_json_string(&json_raw_data_entry)?;
                let short_data_entry = ShortDataEntry::from_raw(&raw_data_entry);
                short_data_entries.push(short_data_entry);
            }

            Ok(short_data_entries)
        })
        .join()
        .unwrap()
    }

    /// `from_tifu_dataset_file_all` creates a `ShortDataEntries` from all the `ShortDataEntry`s in `TIFU_TRAINING_DATA_PATH`.
    pub fn from_tifu_dataset_file_all() -> Result<ShortDataEntries> {
        ShortDataEntries::from_tifu_dataset_file(-1)
    }
}

impl Index<usize> for ShortDataEntries {
    type Output = ShortDataEntry;

    fn index(&self, idx: usize) -> &ShortDataEntry {
        &self.data[idx]
    }
}

impl Iterator for ShortDataEntries {
    type Item = ShortDataEntry;

    fn next(&mut self) -> Option<ShortDataEntry> {
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
    use super::ShortDataEntries;
    use crate::short_data_entry::ShortDataEntry;
    use std::iter::Iterator;

    #[test]
    fn test_short_data_entries_accessors() {
        let mut d1 = ShortDataEntry::new();
        d1.summary = "d1".to_string();
        let mut d2 = ShortDataEntry::new();
        d2.summary = "d2".to_string();
        let mut d3 = ShortDataEntry::new();
        d3.summary = "d3".to_string();
        
        let mut ds = ShortDataEntries::new();
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
    fn test_short_data_entries_modifiers() {
        let d  = ShortDataEntry::new();
        let d1 = d.clone();
        let d2 = d.clone();
        let d3 = d.clone();

        let mut ds = ShortDataEntries::new();
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
    fn test_short_data_entries_from_tifu_dataset_file() {
        let count_1 = 0;
        let count_2 = 10;
        let count_3 = 20;

        let res = ShortDataEntries::from_tifu_dataset_file(count_1);
        assert!(res.is_ok());

        let ds_1 = res.unwrap();
        assert_eq!(ds_1.len(), count_1 as usize);

        let res = ShortDataEntries::from_tifu_dataset_file(count_2);
        assert!(res.is_ok());

        let ds_2 = res.unwrap();
        assert_eq!(ds_2.len(), count_2 as usize);

        let res = ShortDataEntries::from_tifu_dataset_file(count_3);
        assert!(res.is_ok());

        let ds_3 = res.unwrap();
        assert_eq!(ds_3.len(), count_3 as usize);
    }
}
