use super::{CreateUserError, Email, Password};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str, requires2fa: bool) -> Result<User, CreateUserError> {
        let email = Email::parse(email)?;
        let password = Password::parse(password)?;
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
