use curl::easy;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

/// `DEFAULT_DATA_DIR` is the default directory for the dataset(s).
const DEFAULT_DATA_DIR: &str = "data";
/// `TIFU_DATASET_ARCHIVE_FILE` is the TIFU dataset archive filename.
const TIFU_DATASET_ARCHIVE_FILE: &str = "tifu_all_tokenized_and_filtered.zip";
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

/// `tmp_dir_path` builds the `PathBuf` of the temporary directory.
fn tmp_dir_path() -> PathBuf {
    // TODO
    PathBuf::from("tmp")
}

/// `tmp_tifu_dataset_archive_file_path` builds the `PathBuf` of the temporary TIFU dataset archive.
fn tmp_tifu_dataset_archive_file_path<P: AsRef<Path>>(data_dir_path: P) -> PathBuf {
    let mut tmpdir = tmp_dir_path();
    tmpdir.push(data_dir_path);
    tmpdir.push(TIFU_DATASET_ARCHIVE_FILE);
    tmpdir
}

/// `tmp_tifu_dataset_file_path` builds the `PathBuf` of the temporary TIFU dataset.
fn tmp_tifu_dataset_file_path<P: AsRef<Path>>(data_dir_path: P) -> PathBuf {
    let mut tmpdir = tmp_dir_path();
    tmpdir.push(data_dir_path);
    tmpdir.push(TIFU_DATASET_FILE);
    tmpdir
}

/// `tifu_dataset_archive_file_path` builds the `PathBuf` of the TIFU dataset archive.
fn tifu_dataset_archive_file_path<P: AsRef<Path>>(data_dir_path: P) -> PathBuf {
    let mut data_dir = PathBuf::new();
    data_dir.push(data_dir_path);
    data_dir.push(TIFU_DATASET_ARCHIVE_FILE);
    data_dir
}

/// `tifu_dataset_file_path` builds the `PathBuf` of the TIFU dataset.
fn tifu_dataset_file_path<P: AsRef<Path>>(data_dir_path: P) -> PathBuf {
    let mut data_dir = PathBuf::new();
    data_dir.push(data_dir_path);
    data_dir.push(TIFU_DATASET_FILE);
    data_dir
}

/// `fetch_tifu_dataset_archive` fetches the TIFU dataset archive.
fn fetch_tifu_dataset_archive() -> Vec<u8> {
    let mut archive_data = Vec::new();
    let mut curl = easy::Easy::new();

    curl.url(TIFU_DATASET_URL).unwrap();

    {
        let mut transfer = curl.transfer();
        transfer.write_function(|data| {
            archive_data.extend_from_slice(&data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    archive_data
}

fn main() {
    println!("build.rs starting...");

    println!("checking for $DATA_DIR variables, defaulting to $DEFAULT_DATA_DIR if absent...");
    let data_dir = data_dir_from_env();
    let data_dir_path = data_dir_path(&data_dir);
    let tifu_dataset_file_path = tifu_dataset_file_path(&data_dir);
    println!("data directory at {}", data_dir_path.display());
    println!("tifu dataset at {}", tifu_dataset_file_path.display());

    println!("checking if the data directory already exists");
    if dir_exists(&data_dir_path) {
        println!("checking if the TIFU dataset file already exists");
        if file_exists(&tifu_dataset_file_path) {
            return;
        }
    } else {
        println!("building the data directory");
        build_data_dir(data_dir);
    }

    println!("fetching the TIFU dataset archive...");
    let tifu_dataset_archive = fetch_tifu_dataset_archive();

    println!("writing the TIFU dataset archive to tmp/{}", TIFU_DATASET_ARCHIVE_FILE);
    // TODO

    println!("extracting the TIFU dataset archive to tmp/{}...", TIFU_DATASET_FILE);
    // TODO

    println!("writing the TIFU dataset in $DATA_DIR/{}...", TIFU_DATASET_FILE);
    // TODO
    /*
    let tifu_dataset_zipped_path = Path::from(data_dir)
        .join(TIFU_DATASET_ARCHIVED_FILE);
    let file = File::create(tifu_dataset_zipped_path).unwrap();
    */

    println!("build.rs terminated.")
}
