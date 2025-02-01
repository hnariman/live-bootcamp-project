// use thiserror_context::{impl_context, Context};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CreateUserError {
    #[error("Invalid user")]
    InvalidPassword,
    #[error("Invalid email")]
    InvalidEmail,
}
// impl_context!(CreateUserError(CreateUserErrorInner));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn from(email: &str) -> Result<Email, CreateUserError> {
        let valid_email = email.contains('@') && email.contains('.') && (email.len() > 5);
        if !valid_email {
            return Err(CreateUserError::InvalidEmail);
        }
        Ok(Email(email.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn from(pas: &str) -> Result<Password, CreateUserError> {
        let has_numbers = pas.chars().any(|c| c.is_numeric());
        let is_long_enough = pas.len() > 8;

        if !has_numbers || !is_long_enough {
            return Err(CreateUserError::InvalidPassword);
        }

        Ok(Password(pas.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str, requires2fa: bool) -> Result<User, CreateUserError> {
        let email = Email::from(email)?;
        let password = Password::from(password)?;
        let requires_2fa = match Some(requires2fa) {
            Some(val) => val,
            None => true,
        };

        Ok(User {
            email,
            password,
            requires_2fa,
        })
    }
}
