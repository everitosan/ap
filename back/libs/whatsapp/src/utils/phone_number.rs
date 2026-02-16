/// Normalizes a Mexican phone number to 12 digits with "52" country code prefix.
///
/// Handles common formats:
/// - `"5215540128869"` -> `"525540128869"` (removes extra "1" at position 2)
/// - `"+5540128869"`   -> `"525540128869"` (strips "+", prepends "52")
/// - `"5540128869"`    -> `"525540128869"` (prepends "52")
/// - `"525540128869"`  -> `"525540128869"` (already normalized)
pub fn clean(phone: &str) -> String {
    // Strip leading "+"
    let number = phone.strip_prefix('+').unwrap_or(phone);

    // Remove "1" at the third position (index 2) if present.
    // This handles the old Mexican mobile format "521XXXXXXXXXX".
    let number = if number.len() == 13
        && number.starts_with("52")
        && number.as_bytes().get(2) == Some(&b'1')
    {
        format!("52{}", &number[3..])
    } else {
        number.to_string()
    };

    // Ensure 12 digits: prepend "52" if only local 10-digit number
    match number.len() {
        10 => format!("52{number}"),
        _ => number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_plus_and_extra_one() {
        assert_eq!(clean("+5215540128869"), "525540128869");
    }

    #[test]
    fn strips_extra_one_without_plus() {
        assert_eq!(clean("5215540128869"), "525540128869");
    }

    #[test]
    fn prepends_country_code_for_local_number() {
        assert_eq!(clean("5540128869"), "525540128869");
    }

    #[test]
    fn strips_plus_and_prepends_country_code() {
        assert_eq!(clean("+5540128869"), "525540128869");
    }

    #[test]
    fn already_normalized() {
        assert_eq!(clean("525540128869"), "525540128869");
    }

    #[test]
    fn handles_other_lengths_as_is() {
        assert_eq!(clean("123"), "123");
    }
}
