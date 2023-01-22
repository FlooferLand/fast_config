// Gets rid of unnecessary lines
#[cfg(any(feature = "toml", feature = "yaml"))]
pub fn compress_string(string: String) -> String {
    let mut result = String::new();
    for line in string.split('\n') {
        let new_line = line.trim_end();
        if new_line.is_empty() { continue }
        result += format!("{}\n", new_line).as_str();
    }
    result
}
