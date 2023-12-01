use std::path::Path;

pub fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    let input = std::fs::read_to_string(path).unwrap();
    input.lines().map(|s| s.to_owned()).collect()
}
