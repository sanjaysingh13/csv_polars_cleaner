use anyhow::Result;

/// Find start and end line indices of the box of structured data
pub fn detect_data_bounds(lines: &[&str], delimiter: char) -> Result<(usize, usize)> {
    if lines.is_empty() {
        return Err(anyhow::anyhow!("Empty file"));
    }

    // Count columns for each line
    let mut col_counts = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') && trimmed.contains(delimiter) {
            let count = trimmed.split(delimiter).count();
            col_counts.push((i, count));
        }
    }
    if col_counts.is_empty() {
        return Err(anyhow::anyhow!(
            "Could not find any data lines with delimiter"
        ));
    }

    // Find the mode of column counts
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for &(_, count) in &col_counts {
        *freq.entry(count).or_insert(0) += 1;
    }
    let mode_col_count = match freq.iter().max_by_key(|&(_, v)| v) {
        Some((&count, _)) => count,
        None => return Err(anyhow::anyhow!("Could not determine mode column count")),
    };

    // Find first and last line with mode_col_count
    let first = match col_counts.iter().find(|&&(_, c)| c == mode_col_count) {
        Some(&(idx, _)) => idx,
        None => {
            return Err(anyhow::anyhow!(
                "Could not find first line with mode column count"
            ));
        }
    };
    let last = match col_counts.iter().rfind(|&&(_, c)| c == mode_col_count) {
        Some(&(idx, _)) => idx,
        None => {
            return Err(anyhow::anyhow!(
                "Could not find last line with mode column count"
            ));
        }
    };

    if first > last {
        return Err(anyhow::anyhow!("Invalid data bounds: start > end"));
    }

    Ok((first, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bounds() {
        let lines = vec![
            "# Comment",
            "",
            "col1,col2,col3",
            "1,2,3",
            "4,5,6",
            "",
            "# Another comment",
        ];
        let (start, end) = detect_data_bounds(&lines, ',').unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_detect_bounds_with_mixed_content() {
        let lines = vec![
            "# Comment",
            "",
            "col1,col2,col3",
            "1,2,3",
            "",
            "4,5,6",
            "# Another comment",
            "",
        ];
        let (start, end) = detect_data_bounds(&lines, ',').unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_detect_bounds_empty_file() {
        let lines: Vec<&str> = vec![];
        assert!(detect_data_bounds(&lines, ',').is_err());
    }

    #[test]
    fn test_detect_bounds_no_data() {
        let lines = vec!["# Comment", "", "# Another comment"];
        assert!(detect_data_bounds(&lines, ',').is_err());
    }
}
