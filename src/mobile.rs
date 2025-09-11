use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::implementation::KeyringImplementation;
use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_keyring);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Keyring<R>> {
    #[cfg(target_os = "android")]
    {
        use android_native_keyring_store::AndroidStore;
        let store = AndroidStore::from_ndk_context()
            .map_err(|e| crate::Error::PlatformError(e.to_string()))?;
        keyring_core::set_default_store(store);
        let handle =
            api.register_android_plugin("com.alaydriem.bvc.plugin.keyring", "KeyringPlugin")?;
        Ok(Keyring(handle))
    }

    #[cfg(target_os = "ios")]
    {
        use apple_native_keyring_store::protected::Store as IOSStore;
        let store = IOSStore::new().map_err(|e| crate::Error::PlatformError(e.to_string()))?;
        keyring_core::set_default_store(store);
        let handle = api.register_ios_plugin(init_plugin_keyring)?;
        Ok(Keyring(handle))
    }
}

/// Access to the keyring APIs.
pub struct Keyring<R: Runtime>(PluginHandle<R>);

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
