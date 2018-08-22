extern crate serde_json;
extern crate json_gettext;

use json_gettext::Value;

#[test]
fn no_double_quotes() {
    assert_eq!("\"Test\"", serde_json::Value::String("Test".to_string()).to_string());
    assert_eq!("Test", Value::from_json_value(serde_json::Value::String("Test".to_string())).to_string());
}