use core::fmt;

#[derive(Debug, PartialEq)]
pub struct UserError;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Result<Self, UserError> {
        if name.len() == 0 {
            return Err(UserError);
        }
        Ok(User {
            name: name.to_string(),
        })
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_success() {
        let name: &str = "random name";
        let maybe_user: Result<User, UserError> = User::new(name);

        assert!(maybe_user.is_ok());
        let user: User = maybe_user.unwrap();
        assert_eq!(&user.name, name)
    }

    #[test]
    fn test_new_failure() {
        let name: &str = "";
        let maybe_user: Result<User, UserError> = User::new(name);

        assert!(maybe_user.is_err());
        assert_eq!(maybe_user.unwrap_err(), UserError)
    }
}
