use rand::prelude::*;

/// Trait for OTP code generation
pub trait OtpGenerator: Send + Sync {
    fn generate(&self) -> String;
}

/// Random alphanumeric OTP generator
pub struct RandomOtpGenerator;

impl OtpGenerator for RandomOtpGenerator {
    fn generate(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::rng();

        (0..5)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
