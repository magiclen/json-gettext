extern crate serde_json;
extern crate json_gettext;

use json_gettext::Value;

#[test]
fn no_double_quotes() {
    assert_eq!("\"Test\"", serde_json::Value::String("Test".to_string()).to_string());
    assert_eq!("Test", Value::from_json_value(serde_json::Value::String("Test".to_string())).to_string());
}

#[test]
fn escape_double_quotes() {
    assert_eq!("\"Test \\\"abc\\\"\"", serde_json::Value::String("Test \"abc\"".to_string()).to_string());
    assert_eq!("\"Test \\\"abc\\\"\"", Value::from_json_value(serde_json::Value::String("Test \"abc\"".to_string())).to_json());
    assert_eq!("\"Test \\\"abc\\\"\"", Value::from_str("Test \"abc\"").to_json());
}