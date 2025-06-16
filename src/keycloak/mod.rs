pub mod keycloak_config;
pub mod keycloak_registry;

#[derive(thiserror::Error, Debug)]
pub enum KeycloakRegistryError {
    #[error("Keycloak with alias `{0}` already exists")]
    AliasAlreadyExists(String),
    #[error("Config property `{0}` can't be blank")]
    ConfigPropertyCannotBeBlank(&'static str),
    #[error("Can't treat `{url}` as Keycloak URL-address: {origin}")]
    InvalidKeycloakUrl {
        url: String,
        origin: url::ParseError,
    },
    #[error("Can't open config file `{path}`: {origin}")]
    CannotOpenConfigFile {
        path: String,
        origin: std::io::Error,
    },
    #[error("Can't read config file `{path}`: {origin}")]
    CannotUnderstandConfigFile {
        path: String,
        origin: serde_yaml::Error,
    },
    #[error("Can't write config: {origin}")]
    CannotWriteConfigFile { origin: serde_yaml::Error },
}
