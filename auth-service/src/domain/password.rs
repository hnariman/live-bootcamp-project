use validator::HasLen;
use zxcvbn::{zxcvbn, Score};

use super::CreateUserError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn parse(pas: &str) -> Result<Password, CreateUserError> {
        if pas.length() < 8 {
            eprintln!("password too short");
            return Err(CreateUserError::InvalidPassword);
        }

        let password_strength = zxcvbn(pas, &[]);

        if password_strength.score() < Score::Two {
            eprintln!("try a stronger password");
            return Err(CreateUserError::InvalidPassword);
        }

        Ok(Password(pas.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
