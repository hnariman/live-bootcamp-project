use super::CreateUserError;
use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

// TODO: Email validation may fail on different inputs,
// it's better to have erros as verbose as possible
// #[derive(Debug, thiserror::Error)]
// struct EmailError{
//     #[error("Invalid Email {0}")]
//     InvalidEmail(String)
// }

impl Email {
    pub fn parse(email: &str) -> Result<Email, CreateUserError> {
        if validate_email(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(CreateUserError::InvalidEmail)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shall_throw_invalid_email_error_if_no_at() {
        let mail = Email::parse("testingmail.com");
        assert_eq!(mail, Err(CreateUserError::InvalidEmail));
    }

    // FIXME: we're required to use tuple struct and then we need ti
    // fn shall_throw_invalid_email_error_if_no_dot() {
    //     #[test]
    //     let mail = Email::parse("testin@gmailcom");
    //     assert_eq!(mail, Err(CreateUserError::InvalidEmail));
    // }

    #[test]
    fn happy_case() {
        let email = Email::parse("testing@gmail.com");
        assert_eq!(email.unwrap().as_ref(), "testing@gmail.com");
    }
}
