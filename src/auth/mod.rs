pub mod authenticator;

pub trait HttpAuthorization {
    fn get_authorization_value(&self) -> String;
}
