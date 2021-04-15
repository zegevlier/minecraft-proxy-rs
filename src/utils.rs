pub fn make_string_fixed_length(string: String, length: usize) -> String {
    if string.len() <= length {
        return string;
    } else {
        let part_size = length - 3 / 2;
        format!(
            "{}...{}",
            string[0..part_size].to_string(),
            string[string.len() - part_size..].to_string()
        )
    }
}
