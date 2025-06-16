use crate::keycloak::KeycloakRegistryError;
use crate::keycloak::KeycloakRegistryError::{
    CannotOpenConfigFile, CannotUnderstandConfigFile, CannotWriteConfigFile,
};
use crate::keycloak::keycloak_config::KeycloakConfig;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub struct KeycloakRegistry {
    keycloak_by_alias: HashMap<String, KeycloakConfig>, // alias -> keycloak
    default_alias: Option<String>,
}

impl KeycloakRegistry {
    pub fn load_from_file(path: &Path) -> Result<KeycloakRegistry, KeycloakRegistryError> {
        let config_file = File::open(path).map_err(|e| CannotOpenConfigFile {
            path: path.display().to_string(),
            origin: e,
        })?;
        let keycloak_configs: Vec<KeycloakConfig> =
            serde_yaml::from_reader(config_file).map_err(|e| CannotUnderstandConfigFile {
                path: path.display().to_string(),
                origin: e,
            })?;
        let mut keycloaks = Self::new_empty();
        let mut default_alias = None;
        for config in keycloak_configs {
            if config.default.get() {
                default_alias = Some(config.alias.clone());
            }
            keycloaks.add_keycloak(config)?;
        }
        keycloaks.default_alias = default_alias;
        Ok(keycloaks)
    }

    pub fn save_to_file(self, path: &Path) -> Result<(), KeycloakRegistryError> {
        let config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| CannotOpenConfigFile {
                path: path.display().to_string(),
                origin: e,
            })?;
        let keycloak_configs: Vec<&KeycloakConfig> = self.keycloak_by_alias.values().collect();
        serde_yaml::to_writer(config_file, &keycloak_configs)
            .map_err(|e| CannotWriteConfigFile { origin: e })
    }

    pub fn new_empty() -> Self {
        Self {
            keycloak_by_alias: HashMap::new(),
            default_alias: None,
        }
    }

    pub fn add_keycloak(&mut self, config: KeycloakConfig) -> Result<(), KeycloakRegistryError> {
        log::info!("Adding keycloak for config `{:?}`", config);
        let alias = config.alias.to_owned();
        let is_default = config.default.get();
        if self.keycloak_by_alias.contains_key(&config.alias) {
            return Err(KeycloakRegistryError::AliasAlreadyExists(alias));
        }
        self.keycloak_by_alias.insert(alias.to_owned(), config);
        if is_default {
            self.set_default(&alias);
        }
        Ok(())
    }

    pub fn remove_keycloak(&mut self, alias: &str) {
        log::info!("Removing keycloak for alias `{}`", alias);
        self.keycloak_by_alias.remove(alias);
    }

    pub fn find_keycloak(&self, alias: &str) -> Option<&KeycloakConfig> {
        self.keycloak_by_alias.get(alias)
    }

    pub fn get_all(&self) -> Vec<&KeycloakConfig> {
        self.keycloak_by_alias.values().collect()
    }

    pub fn get_default(&self) -> Option<&KeycloakConfig> {
        let default = self.default_alias.as_ref()?;
        self.keycloak_by_alias.get(default)
    }

    pub fn set_default(&mut self, new_default_alias: &str) {
        let is_default_the_same = self
            .default_alias
            .as_ref()
            .map(|curr| curr == new_default_alias)
            .unwrap_or(false);
        if !is_default_the_same {
            // unset current default
            self.get_default()
                .iter()
                .for_each(|curr_default| curr_default.default.set(false));
            // set new default
            self.find_keycloak(new_default_alias)
                .iter()
                .for_each(|new_default| new_default.default.set(true));
            // set cached value
            self.default_alias = Some(new_default_alias.to_string());
        }
    }
}
