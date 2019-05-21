use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
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
        self.data.push(entry);
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
    pub fn from_tifu_training_data(count: i32) -> Result<DataEntries> {
        thread::spawn(move || {
            let path = tifu_training_data_path();
            let file = File::open(&path).map_err(|e| format!("{}", e))?;
            let reader = BufReader::new(file);
            let mut data_entries = DataEntries::new();

            for (i, line) in reader.lines().enumerate() {
                if i as i32 == count {
                    break;
                }

                let json_data_entry = line.unwrap();
                let data_entry = DataEntry::from_json_string(&json_data_entry)?;
                data_entries.push(data_entry);
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
    use super::DataEntries;
    use crate::data_entry::DataEntry;
    use crate::path::tifu_training_data_path;
    use std::iter::Iterator;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_data_entries_accessors() {
        let mut d1 = DataEntry::new();
        d1.title = "d1".to_string();
        let mut d2 = DataEntry::new();
        d2.title = "d2".to_string();
        let mut d3 = DataEntry::new();
        d3.title = "d3".to_string();
        
        let mut ds = DataEntries::new();
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
    fn test_data_entries_tifu_datasets() {
        let count_1 = 0;
        let count_2 = 10;
        let count_3 = 20;

        let res = DataEntries::from_tifu_training_data(count_1);
        assert!(res.is_ok());

        let ds_1 = res.unwrap();
        assert_eq!(ds_1.len(), count_1 as usize);

        let res = DataEntries::from_tifu_training_data(count_2);
        assert!(res.is_ok());

        let ds_2 = res.unwrap();
        assert_eq!(ds_2.len(), count_2 as usize);

        let res = DataEntries::from_tifu_training_data(count_3);
        assert!(res.is_ok());

        let ds_3 = res.unwrap();
        assert_eq!(ds_3.len(), count_3 as usize);
    }
}
