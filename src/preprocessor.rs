/// Normalizes line endings in the input string to Unix-style (\n)
/// This ensures consistent line endings regardless of the input format
pub fn normalize_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}
