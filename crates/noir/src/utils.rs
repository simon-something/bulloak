//! Utility functions for Noir code generation.

/// Convert a title to snake_case, stripping BDD prefixes.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(to_snake_case("When user is logged in"), "user_is_logged_in");
/// assert_eq!(to_snake_case("It should return true"), "should_return_true");
/// ```
pub(crate) fn to_snake_case(title: &str) -> String {
    // Strip BDD prefixes
    let stripped = title
        .trim()
        .trim_start_matches("when ")
        .trim_start_matches("given ")
        .trim_start_matches("it ")
        .trim_start_matches("When ")
        .trim_start_matches("Given ")
        .trim_start_matches("It ");

    // Convert to snake_case
    stripped
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() {
                Some('_')
            } else {
                None
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(
            to_snake_case("When user is logged in"),
            "user_is_logged_in"
        );
        assert_eq!(
            to_snake_case("It should return true"),
            "should_return_true"
        );
        assert_eq!(to_snake_case("given amount is zero"), "amount_is_zero");
        assert_eq!(
            to_snake_case("When first arg is bigger than second arg"),
            "first_arg_is_bigger_than_second_arg"
        );
    }

    #[test]
    fn test_to_snake_case_with_special_chars() {
        assert_eq!(to_snake_case("It's working!"), "its_working");
        assert_eq!(to_snake_case("value > 100"), "value_100");
    }
}
