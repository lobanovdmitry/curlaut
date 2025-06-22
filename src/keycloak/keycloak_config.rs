use crate::keycloak::KeycloakRegistryError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakConfig {
    pub alias: String,
    pub url: MyUrl, // to add custom serde
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub(in crate::keycloak) default: bool,
}

impl std::fmt::Display for KeycloakConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.default {
            write!(f, "* ")?;
        }
        write!(f, "Keycloak '{}'", self.alias)?;
        write!(
            f,
            ":\n  [ url={}, realm={}, client_id={}, client_secret={}, username={}, password={} ]",
            self.url.0.to_string(),
            self.realm,
            self.client_id,
            self.client_secret,
            self.username,
            self.password
        )
    }
}

#[derive(Debug)]
pub struct MyUrl(pub Url); // just NewType for Url serde

impl Serialize for MyUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for MyUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let url = Url::parse(&s).map_err(serde::de::Error::custom)?;
        Ok(Self(url))
    }
}

impl KeycloakConfig {
    pub fn new(
        alias: &str,
        url: &str,
        realm: &str,
        client_id: &str,
        client_secret: &str,
        username: &str,
        password: &str,
        default: bool,
    ) -> Result<KeycloakConfig, KeycloakRegistryError> {
        let config = KeycloakConfig {
            alias: Self::require_non_empty("alias", alias)?,
            url: MyUrl(Self::require_url(url)?),
            realm: Self::require_non_empty("realm", realm)?,
            client_id: Self::require_non_empty("client_id", client_id)?,
            client_secret: client_secret.to_string(), // empty string is ok
            username: Self::require_non_empty("username", username)?,
            password: Self::require_non_empty("password", password)?,
            default,
        };
        Ok(config)
    }

    fn require_non_empty(
        property: &'static str,
        value: &str,
    ) -> Result<String, KeycloakRegistryError> {
        if value.is_empty() {
            return Err(KeycloakRegistryError::ConfigPropertyCannotBeBlank(property));
        }
        Ok(value.to_owned())
    }

    fn require_url(url: &str) -> Result<Url, KeycloakRegistryError> {
        Url::parse(url).map_err(|e| KeycloakRegistryError::InvalidKeycloakUrl {
            url: url.to_owned(),
            origin: e,
        })
    }
}
