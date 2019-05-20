use std::env;

/// `DEFAULT_DATA_OUT` is the default directory for the dataset(s).
const DEFAULT_DATA_OUT: &str = "data";
/// `TIFU_DATASET_ARCHIVE_FILENAME` is the TIFU dataset archive filename.
const TIFU_DATASET_ARCHIVE_FILENAME: &str = "tifu_all_tokenized_and_filtered.zip";
/// `TIFU_DATASET_FILENAME` is the TIFU dataset filename.
const TIFU_DATASET_FILENAME: &str = "tifu_all_tokenized_and_filtered.json";
/// `TIFU_DATASET_URL` is the TIFU dataset url.
const TIFU_DATASET_URL: &str = "https://github.com/chritchens/mmn_dataset/raw/master/tifu_all_tokenized_and_filtered.zip";

fn main() {
    println!("build.rs starting...");

    println!("checking for $DATA_OUT variables...");
    let data_out = env::var("DATA_OUT").unwrap_or(DEFAULT_DATA_OUT.to_string());
    println!("$DATA_OUT at {}", data_out);

    println!("building $DATA_OUT if absent...");

    println!("fetching the TIFU dataset...");

    println!("decompressing the TIFU dataset...");

    println!("writing the TIFU dataset in $DATA_OUT/{}...", TIFU_DATASET_FILENAME);

    println!("build.rs terminated.")
}
