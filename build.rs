use curl::easy::{Easy2, Handler, WriteError};
use zip::ZipArchive;
use std::path::{Path, PathBuf};
use std::env;
use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::thread;
use std::sync::mpsc::channel;

/// `DEFAULT_DATA_DIR` is the default directory for the dataset(s).
const DEFAULT_DATA_DIR: &str = "data";
/// `TIFU_DATASET_FILE` is the TIFU dataset filename.
const TIFU_DATASET_FILE: &str = "tifu_all_tokenized_and_filtered.json";
/// `TIFU_DATASET_URL` is the TIFU dataset url.
const TIFU_DATASET_URL: &str = "https://github.com/chritchens/mmn_dataset/raw/master/data/tifu_all_tokenized_and_filtered.zip";

/// `dir_exists` returns if a directory already exists.
fn dir_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|meta| meta.is_dir())
        .unwrap_or(false)
}

/// `file_exists` returns if a file already exists.
fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|meta| meta.is_file())
        .unwrap_or(false)
}

/// `data_dir_from_env` returns the data directory name from env or the default `DEFAULT_DATA_DIR`.
fn data_dir_from_env() -> String {
    env::var("DATA_DIR").unwrap_or(DEFAULT_DATA_DIR.to_string())
}

/// `data_dir_path` builds the `PathBuf` of the data directory.
fn data_dir_path<P: AsRef<Path>>(data_dir: P) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(data_dir);
    path
}

/// `build_data_dir` creates the data directory.
fn build_data_dir<P: AsRef<Path>>(data_dir: P) {
    fs::create_dir(data_dir).unwrap()
}

/// `tifu_dataset_file_path` builds the `PathBuf` of the TIFU dataset.
fn tifu_dataset_file_path<P: AsRef<Path>>(data_dir_path: P) -> PathBuf {
    let mut data_dir = PathBuf::new();
    data_dir.push(data_dir_path);
    data_dir.push(TIFU_DATASET_FILE);
    data_dir
}

/// `TIFUDatasetArchiveHandler` is the `curl::easy::Handler` used by `curl::easy::Easy2` to
/// write the archive data
struct TIFUDatasetArchiveHandler {
    bytes: usize,
    contents: Vec<u8>,
}

impl TIFUDatasetArchiveHandler {
    /// `new` creates a new instance of `TIFUDatasetArchiveHandler`.
    fn new() -> TIFUDatasetArchiveHandler {
        TIFUDatasetArchiveHandler { bytes: 0, contents: Vec::new() }
    }

    /// `size` returns the actual size of the inner contents of the `TIFUDatasetArchiveHandler`.
    fn size(&self) -> usize {
        self.bytes
    }

    /// `drain` returns the inner contents of the `TIFUDatasetArchiveHandler`
    /// removing them from the handler.
    fn drain(&mut self) -> Vec<u8> {
        let contents = self.contents.to_owned();
        self.contents.clear();
        contents
    }
}

impl Handler for TIFUDatasetArchiveHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.contents.extend_from_slice(data);
        let size = data.len();
        self.bytes += size;
        let tot_size = self.size();
        println!("TIFU dataset archive: fetched {} bytes, tot bytes: {} of 141774983", size, tot_size);
        Ok(size)
    }
}

/// `fetch_tifu_dataset_archive` fetches the TIFU dataset archive.
fn fetch_tifu_dataset_archive() -> Vec<u8> {
    let mut curl = Easy2::new(TIFUDatasetArchiveHandler::new());

    curl.url(TIFU_DATASET_URL).unwrap();
    curl.get(true).unwrap();
    curl.follow_location(true).unwrap();

    curl.perform().unwrap();

    curl.get_mut().drain()
}

/// `extract_tifu_dataset_archive` extracts the TIFU dataset archive.
fn extract_tifu_dataset_archive(archive_buf: &[u8]) -> Vec<u8> {
    let mut tifu_dataset = Vec::new();
    let reader = Cursor::new(archive_buf);
    let mut zip_archive = ZipArchive::new(reader).unwrap();
    let mut zip_file = zip_archive.by_name(TIFU_DATASET_FILE).unwrap();
    zip_file.read_to_end(&mut tifu_dataset).unwrap();
    tifu_dataset
}

fn main() {
    println!("build.rs starting...");

    println!("checking for $DATA_DIR variables, defaulting to $DEFAULT_DATA_DIR if absent...");
    let data_dir = data_dir_from_env();
    let data_dir_path = data_dir_path(&data_dir);
    let tifu_dataset_file_path = tifu_dataset_file_path(&data_dir);
  
    println!("data directory path set at '{}'", data_dir_path.display());
    println!("tifu dataset file path set at '{}'", tifu_dataset_file_path.display());

    println!("checking if the data directory already exists...");
    if dir_exists(&data_dir_path) {
        println!("the data directory already exists...");
        println!("checking if the TIFU dataset file already exists...");
        if file_exists(&tifu_dataset_file_path) {
            println!("the TIFU dataset already exists");
            return;
        }

        println!("the TIFU dataset does not exist");
    } else {
        println!("the data directory does not exist");
        println!("building the data directory...");
        build_data_dir(data_dir);
    }

    let tifu_dataset = thread::spawn(move || {
        let (extract_sender, extract_receiver) = channel();

        thread::spawn(move || {
            println!("fetching the TIFU dataset archive...");
            let tifu_dataset_archive = fetch_tifu_dataset_archive();
            extract_sender.send(tifu_dataset_archive).unwrap()
        })
        .join()
        .unwrap();

        println!("extracting the TIFU dataset archive...");
        let tifu_dataset_archive = extract_receiver.recv().unwrap();
        extract_tifu_dataset_archive(&tifu_dataset_archive)
    })
    .join()
    .unwrap();

    println!("writing the TIFU dataset into '{}'...", tifu_dataset_file_path.display());
    let mut tifu_dataset_file = File::create(tifu_dataset_file_path).unwrap();
    tifu_dataset_file.write_all(&tifu_dataset).unwrap();

    println!("build.rs terminated.")
}
