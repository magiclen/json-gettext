extern crate json_gettext;
extern crate serde_json;

use json_gettext::JSONGetTextValue;

#[test]
fn no_double_quotes() {
    assert_eq!("\"Test\"", serde_json::Value::String("Test".to_string()).to_string());
    assert_eq!(
        "Test",
        JSONGetTextValue::from_json_value(serde_json::Value::String("Test".to_string()))
            .to_string()
    );
}

#[test]
fn escape_double_quotes() {
    assert_eq!(
        "\"Test \\\"abc\\\"\"",
        serde_json::Value::String("Test \"abc\"".to_string()).to_string()
    );
    assert_eq!(
        "\"Test \\\"abc\\\"\"",
        JSONGetTextValue::from_json_value(serde_json::Value::String("Test \"abc\"".to_string()))
            .to_json()
    );
    assert_eq!("\"Test \\\"abc\\\"\"", JSONGetTextValue::from_str("Test \"abc\"").to_json());
}
