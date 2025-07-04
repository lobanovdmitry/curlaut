use crate::keycloak::keycloak_config::KeycloakConfig;
use anyhow::{Context, bail};
use linked_hash_map::LinkedHashMap;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub struct KeycloakRegistry {
    keycloak_by_alias: LinkedHashMap<String, KeycloakConfig>, // alias -> keycloak
    default_alias: Option<String>,
}

impl KeycloakRegistry {
    pub fn load_from_file(path: &Path) -> anyhow::Result<KeycloakRegistry> {
        let config_file = File::open(path).with_context(|| "Can't open config file for read")?;
        let keycloak_configs: Vec<KeycloakConfig> =
            serde_yaml::from_reader(config_file).with_context(|| "Can't parse config file")?;
        let mut keycloaks = Self::new_empty();
        let mut default_alias = None;
        for config in keycloak_configs {
            if config.default {
                default_alias = Some(config.alias.clone());
            }
            keycloaks.add_keycloak(config)?;
        }
        keycloaks.default_alias = default_alias;
        Ok(keycloaks)
    }

    pub fn save_to_file(self, path: &Path) -> anyhow::Result<()> {
        let config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .with_context(|| "Can't open config file for write")?;
        let keycloak_configs: Vec<&KeycloakConfig> = self.keycloak_by_alias.values().collect();
        serde_yaml::to_writer(config_file, &keycloak_configs)
            .with_context(|| "Can't write config file")?;
        Ok(())
    }

    pub fn new_empty() -> Self {
        Self {
            keycloak_by_alias: LinkedHashMap::new(),
            default_alias: None,
        }
    }

    pub fn add_keycloak(&mut self, config: KeycloakConfig) -> anyhow::Result<()> {
        log::info!("Adding keycloak for config `{:?}`", config);
        let alias = config.alias.to_owned();
        let is_default = config.default;
        if self.keycloak_by_alias.contains_key(&config.alias) {
            bail!("Alias `{}` already exists", alias);
        }
        self.keycloak_by_alias.insert(alias.to_owned(), config);
        if is_default {
            self.set_default(&alias)?;
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

    pub fn set_default(&mut self, new_default_alias: &str) -> anyhow::Result<()> {
        let is_default_the_same = self
            .default_alias
            .as_ref()
            .map(|curr| curr == new_default_alias)
            .unwrap_or(false);
        if !is_default_the_same {
            // unset current default
            if let Some(current_default) = self.get_default_mut() {
                current_default.default = false;
            }
            // set new default
            let new_default = self
                .keycloak_by_alias
                .get_mut(new_default_alias)
                .with_context(|| format!("Keycloak with alias `{new_default_alias}` not found`"))?;
            new_default.default = true;
            // set cached value
            self.default_alias = Some(new_default_alias.to_string());
        }
        Ok(())
    }

    fn get_default_mut(&mut self) -> Option<&mut KeycloakConfig> {
        self.default_alias
            .as_ref()
            .and_then(|alias| self.keycloak_by_alias.get_mut(alias))
    }
}
