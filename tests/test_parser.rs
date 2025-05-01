use csv_polars_cleaner::parse_boxed_data;
use polars::prelude::*;

/// Test helper function to create a DataFrame with expected data
fn create_expected_df() -> DataFrame {
    df!(
        "name" => &["Alice", "Bob", "Charlie"],
        "age" => &["30", "25", "35"],
        "city" => &["New York", "London", "Paris"]
    )
    .unwrap()
}

#[test]
fn test_parse_with_comments_and_empty_lines() {
    let input = r#"# This is a comment
# Another comment

name,age,city
Alice,30,New York
Bob,25,London
Charlie,35,Paris

# End of file"#;

    let result = parse_boxed_data(input, b',');
    assert!(
        result.is_ok(),
        "Failed to parse CSV with comments and empty lines"
    );

    let df = result.unwrap();
    let expected = create_expected_df();
    assert!(df.equals(&expected), "DataFrame content mismatch");
}

#[test]
fn test_parse_with_extra_whitespace() {
    let input = r#"  name  ,  age  ,  city  
  Alice  ,  30  ,  New York  
  Bob  ,  25  ,  London  
  Charlie  ,  35  ,  Paris  "#;

    let result = parse_boxed_data(input, b',');
    assert!(result.is_ok(), "Failed to parse CSV with extra whitespace");

    let df = result.unwrap();
    let expected = create_expected_df();
    assert!(df.equals(&expected), "DataFrame content mismatch");
}

#[test]
fn test_parse_with_mixed_line_endings() {
    let input = "name,age,city\r\nAlice,30,New York\nBob,25,London\nCharlie,35,Paris";

    let result = parse_boxed_data(input, b',');
    assert!(
        result.is_ok(),
        "Failed to parse CSV with mixed line endings"
    );

    let df = result.unwrap();
    let expected = create_expected_df();
    assert!(df.equals(&expected), "DataFrame content mismatch");
}

#[test]
fn test_parse_with_tab_delimiter() {
    let input = r#"name	age	city
Alice	30	New York
Bob	25	London
Charlie	35	Paris"#;

    let result = parse_boxed_data(input, b'\t');
    assert!(result.is_ok(), "Failed to parse CSV with tab delimiter");

    let df = result.unwrap();
    let expected = create_expected_df();
    assert!(df.equals(&expected), "DataFrame content mismatch");
}

#[test]
fn test_parse_empty_file() {
    let input = "";
    let result = parse_boxed_data(input, b',');
    assert!(result.is_err(), "Should fail on empty file");
}

#[test]
fn test_parse_only_comments() {
    let input = r#"# Comment 1
# Comment 2
# Comment 3"#;
    let result = parse_boxed_data(input, b',');
    assert!(result.is_err(), "Should fail on file with only comments");
}
