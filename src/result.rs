/// `Result` is an alias of the std library `std::result::Result` with `String` as `Error` type.
pub type Result<T> = std::result::Result<T, String>;
