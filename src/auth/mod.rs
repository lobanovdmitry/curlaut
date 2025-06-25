use std::fmt::Display;

pub mod authenticator;

pub trait HttpAuthorization: Display {
    fn get_authorization_value(&self) -> String;
}
