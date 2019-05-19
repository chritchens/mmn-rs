use std::path::PathBuf;

/// `DATADIR_PATH` is the path of the data directory from root.
pub const DATADIR_PATH: &str = "data";
/// `TIFU_TRAINING_DATA_PATH` is the path of the default tifu training data json file in the data directory.
pub const TIFU_TRAINING_DATA_PATH: &str = "tifu_all_tokenized_and_filtered.json";

/// `data_dir_path` returns the `PathBuf` of `DATADIR_PATH`.
pub fn data_dir_path() -> PathBuf {
    PathBuf::from(DATADIR_PATH)
}

/// `tifu_training_data_path` returns the `PathBuf` of `TIFU_TRAINING_DATA_TRAINING`.
pub fn tifu_training_data_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(DATADIR_PATH);
    path.push(TIFU_TRAINING_DATA_PATH);
    path
}
