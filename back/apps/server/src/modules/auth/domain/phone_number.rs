use crate::shared::AppError;

/// Value Object for Mexican phone numbers
/// Invariants:
/// - Exactly 10 digits
/// - Only numeric characters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: &str) -> Result<Self, AppError> {
        let cleaned: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

        if cleaned.len() != 10 {
            return Err(AppError::BadRequest(
                "Phone number must be exactly 10 digits".to_string(),
            ));
        }

        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::BadRequest(
                "Phone number must contain only digits".to_string(),
            ));
        }

        Ok(Self(cleaned))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_phone() {
        let phone = PhoneNumber::new("5512345678");
        assert!(phone.is_ok());
        assert_eq!(phone.unwrap().value(), "5512345678");
    }

    #[test]
    fn cleans_spaces() {
        let phone = PhoneNumber::new("55 1234 5678");
        assert!(phone.is_ok());
        assert_eq!(phone.unwrap().value(), "5512345678");
    }

    #[test]
    fn invalid_length() {
        let phone = PhoneNumber::new("123456789");
        assert!(phone.is_err());
    }
}
