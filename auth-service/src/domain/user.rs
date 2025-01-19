#[derive(Debug)]
pub enum CreateUserError {
    InvalidPassword,
    InvalidEmail,
    InvalidName,
}

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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str) -> Result<User, CreateUserError> {
        let email = Email::from(email)?;
        let password = Password::from(password)?;

        Ok(User {
            email,
            password,
            requires_2fa: true,
        })
    }
}

// struct Check {}
//
// impl Check {
//     fn is_valid_email(email: &str) -> Result<&str, CreateUserError> {
//         let valid_email = email.contains('@') && email.contains('.') && (email.len() > 5);
//         if !valid_email {
//             return Err(CreateUserError::InvalidEmail);
//         }
//         Ok(email)
//     }
//
//     fn is_valid_password(pas: &str) -> Result<&str, CreateUserError> {
//         let has_numbers = pas.chars().any(|c| c.is_numeric());
//         let is_long_enough = pas.len() > 8;
//
//         if !has_numbers || !is_long_enough {
//             return Err(CreateUserError::InvalidPassword);
//         }
//
//         Ok(pas)
//     }
// }
