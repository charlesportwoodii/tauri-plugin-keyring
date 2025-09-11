use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::implementation::KeyringImplementation;
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Keyring<R>> {
    if std::env::var("KEYRING_USE_MOCK").is_ok() {
        use keyring_core::mock::Store;
        let store = Store::new().map_err(|e| crate::Error::PlatformError(e.to_string()))?;
        keyring_core::set_default_store(store);
        return Ok(Keyring(app.clone()));
    }

    // Initialize platform-specific store
    #[cfg(target_os = "windows")]
    {
        use windows_native_keyring_store::Store as WindowsStore;
        let store = WindowsStore::new().map_err(|e| crate::Error::PlatformError(e.to_string()))?;
        keyring_core::set_default_store(store);
    }

    #[cfg(target_os = "macos")]
    {
        use apple_native_keyring_store::keychain::Store as MacOSStore;
        let store = MacOSStore::new().map_err(|e| crate::Error::PlatformError(e.to_string()))?;
        keyring_core::set_default_store(store);
    }

    #[cfg(target_os = "linux")]
    {
        #[cfg(feature = "dbus-secret-service")]
        use dbus_secret_service_keyring_store::Store as LinuxStore;
        #[cfg(feature = "linux-keyutils")]
        use linux_keyutils_keyring_store::Store as LinuxStore;
        let store = LinuxStore::new().map_err(|e| crate::Error::PlatformError(e.to_string()))?;
        keyring_core::set_default_store(store);
    }

    Ok(Keyring(app.clone()))
}

/// Access to the keyring APIs.
pub struct Keyring<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Keyring<R> {
    fn implementation(&self) -> KeyringImplementation {
        KeyringImplementation
    }

    pub fn initialize_service(&self, service_name: String) -> crate::Result<()> {
        KeyringImplementation::initialize_service(service_name)
    }

    pub fn set(
        &self,
        username: &str,
        credential_type: CredentialType,
        value: CredentialValue,
    ) -> crate::Result<()> {
        self.implementation().set(username, credential_type, value)
    }

    pub fn get(
        &self,
        username: &str,
        credential_type: CredentialType,
    ) -> crate::Result<CredentialValue> {
        self.implementation().get(username, credential_type)
    }

    pub fn delete(&self, username: &str, credential_type: CredentialType) -> crate::Result<()> {
        self.implementation().delete(username, credential_type)
    }

    pub fn exists(&self, username: &str, credential_type: CredentialType) -> crate::Result<bool> {
        self.implementation().exists(username, credential_type)
    }
}
