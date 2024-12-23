use std::fs;
pub fn read_from_file(filename: &str) -> String {
    println!("reading {}", filename);
    let result = fs::read_to_string(filename)
        .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg));
    result.replace("\r\n", "\n")
}

#[macro_export]
macro_rules! local_file {
    ($file:literal) => {
        LazyLock::new(|| common::test_utils::read_from_file(&format!("{}", $file)))
    };
}
// re-export macro such that test_utils::local_file path can be used.
pub use crate::local_file;
