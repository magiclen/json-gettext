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
            .to_json_string()
    );
    assert_eq!(
        "\"Test \\\"abc\\\"\"",
        JSONGetTextValue::from_str_ref("Test \"abc\"").to_json_string()
    );
}

#[test]
fn escape_control_characters() {
    // Control characters must be escaped as `\u00XX`, producing valid JSON.
    assert_eq!(
        "\"\\u0000\\u0007\\n\"",
        JSONGetTextValue::from_str_ref("\u{0}\u{7}\n").to_json_string()
    );
}
