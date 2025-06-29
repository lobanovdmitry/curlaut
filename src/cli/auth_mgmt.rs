use crate::cli::clap_config::KeycloakCommand;
use crate::cli::clap_config::KeycloakCommand::SetDefault;
use crate::cli::auth_config_file_path;
use crate::keycloak::keycloak_config::KeycloakConfig;
use crate::keycloak::keycloak_registry::KeycloakRegistry;
use crate::output::CurlautOutput;
use std::io::Write;
use anyhow::Context;
use KeycloakCommand::{Add, List, Remove};

pub fn execute_command(
    command: KeycloakCommand,
    io: &mut impl CurlautOutput,
) -> anyhow::Result<()> {
    let config_file_path = auth_config_file_path()?;
    let config_file_path = config_file_path.as_path();
    let mut keycloak_registry = KeycloakRegistry::load_from_file(config_file_path)?;
    match &command {
        Add {
            alias,
            url,
            realm,
            client_id,
            client_secret,
            username,
            password,
            default,
        } => {
            writeln!(io.common(), "Adding keycloak with alias: {alias}")?;
            let result = KeycloakConfig::new(
                alias,
                url,
                realm,
                client_id,
                client_secret,
                username,
                password,
                *default,
            )
            .with_context(||"Failed to create keycloak config")?;
            keycloak_registry.add_keycloak(result)?;
            keycloak_registry.save_to_file(config_file_path)?;
            Ok(())
        }
        Remove { alias } => {
            writeln!(io.common(), "Removing keycloak by alias {alias}")?;
            keycloak_registry.remove_keycloak(alias);
            keycloak_registry.save_to_file(config_file_path)?;
            Ok(())
        }
        SetDefault { alias } => {
            writeln!(io.common(), "Set keycloak by default alias {alias}")?;
            keycloak_registry.set_default(alias)?;
            keycloak_registry.save_to_file(config_file_path)?;
            Ok(())
        }
        List {} => {
            for kc in keycloak_registry.get_all() {
                writeln!(io.common(), "{kc}")?;
            }
            Ok(())
        }
    }
}
