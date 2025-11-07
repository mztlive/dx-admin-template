/// Merge a base CSS class with an optional extra class string.
/// If the extra string is empty or only whitespace, returns just the base class.
pub fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

/// Convert a boolean to "true" or "false" string for data attributes.
pub fn data_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_class_with_extra() {
        assert_eq!(
            merge_class("base", Some("extra".to_string())),
            "base extra"
        );
    }

    #[test]
    fn test_merge_class_without_extra() {
        assert_eq!(merge_class("base", None), "base");
    }

    #[test]
    fn test_merge_class_with_empty_extra() {
        assert_eq!(merge_class("base", Some("".to_string())), "base");
        assert_eq!(merge_class("base", Some("   ".to_string())), "base");
    }

    #[test]
    fn test_data_bool() {
        assert_eq!(data_bool(true), "true");
        assert_eq!(data_bool(false), "false");
    }
}
