# Tauri Plugin Keyring

![Tauri Plugin Keyring](banner.png)

This plugin provides cross-platform keyring/keychain access for Tauri applications, enabling secure storage and retrieval of passwords, API keys, and other sensitive data using platform-native secure storage mechanisms.

## Platform Support

| Platform | Support |
|----------|---------|
| Linux    | ✓       |
| Windows  | ✓       |
| macOS    | ✓       |
| Android  | ✓       |
| iOS      | ✓       |

## Security Features

- **Windows**: Uses Windows Credential Manager with hardware-backed security when available
- **macOS/iOS**: Leverages Apple Keychain Services with Secure Enclave support
- **Android**: Uses Android Keystore system with hardware security module backing
- **Linux**: Uses dbus-secret-service by default, but keyutils is also available
- **Cross-platform**: Unified API with platform-specific optimizations

## Install

This plugin requires a Rust version of at least **1.77.2**.

There are three general methods of installation that we can recommend:

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`
```toml
[dependencies]
tauri-plugin-keyring = "0.1.0"

# alternatively with Git:
tauri-plugin-keyring = { git = "https://github.com/charlesportwoodii/tauri-plugin-keyring", branch = "master" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```bash
pnpm add tauri-plugin-keyring-api
# or
npm add tauri-plugin-keyring-api
# or
yarn add tauri-plugin-keyring-api
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/lib.rs`
```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Then, grant the plugin the necessary permissions in your capabilities configuration:

`src-tauri/capabilities/default.json`
```json
{
  "permissions": [
    "core:default",
    "keyring:allow-initialize-keyring",
    "keyring:allow-set-password",
    "keyring:allow-get-password",
    "keyring:allow-delete-password",
    "keyring:allow-has-password",
    "keyring:allow-set-secret",
    "keyring:allow-get-secret",
    "keyring:allow-delete-secret",
    "keyring:allow-has-secret"
  ]
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```typescript
import {
  initializeKeyring,
  setPassword,
  getPassword,
  deletePassword,
  hasPassword,
  setSecret,
  getSecret,
  deleteSecret,
  hasSecret
} from 'tauri-plugin-keyring'

// Initialize the keyring with your service name
await initializeKeyring('com.example.myapp')

// Store a password
await setPassword('user@example.com', 'mysecretpassword')

// Retrieve a password
const password = await getPassword('user@example.com')
console.log('Retrieved password:', password)

// Check if a password exists
const exists = await hasPassword('user@example.com')
console.log('Password exists:', exists)

// Store binary secret data
const secretData = new TextEncoder().encode('sensitive data')
await setSecret('api-key', Array.from(secretData))

// Retrieve binary secret data
const retrievedSecret = await getSecret('api-key')
const secretString = new TextDecoder().decode(new Uint8Array(retrievedSecret))
console.log('Retrieved secret:', secretString)

// Delete credentials
await deletePassword('user@example.com')
await deleteSecret('api-key')
```

## Rust Usage

The plugin can also be used directly from Rust code:

```rust
use tauri_plugin_keyring::KeyringExt;

#[tauri::command]
async fn store_user_token(app: tauri::AppHandle, user_id: String, token: String) -> Result<(), String> {
    // Initialize keyring for your service
    app.keyring().initialize_service("com.example.myapp".to_string())
        .map_err(|e| e.to_string())?;

    // Store the token securely
    app.keyring().set(
        &user_id,
        tauri_plugin_keyring::CredentialType::Password,
        tauri_plugin_keyring::CredentialValue::Password(token)
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_user_token(app: tauri::AppHandle, user_id: String) -> Result<String, String> {
    match app.keyring().get(&user_id, tauri_plugin_keyring::CredentialType::Password) {
        Ok(tauri_plugin_keyring::CredentialValue::Password(token)) => Ok(token),
        Err(e) => Err(e.to_string()),
        _ => Err("Invalid credential type".to_string()),
    }
}
```

## API Reference

### JavaScript/TypeScript API

#### `initializeKeyring(serviceName: string): Promise<void>`
Initialize the keyring with a service name. This should be called once before using other keyring operations.

#### `setPassword(username: string, password: string): Promise<void>`
Store a password for the given username.

#### `getPassword(username: string): Promise<string>`
Retrieve a password for the given username.

#### `deletePassword(username: string): Promise<void>`
Delete the password for the given username.

#### `hasPassword(username: string): Promise<boolean>`
Check if a password exists for the given username.

#### `setSecret(username: string, secret: number[]): Promise<void>`
Store binary secret data for the given username. The secret should be provided as an array of bytes.

#### `getSecret(username: string): Promise<number[]>`
Retrieve binary secret data for the given username. Returns an array of bytes.

#### `deleteSecret(username: string): Promise<void>`
Delete the secret for the given username.

#### `hasSecret(username: string): Promise<boolean>`
Check if a secret exists for the given username.

### Rust API

The Rust API provides access to keyring operations through the `KeyringExt` trait:

```rust
pub trait KeyringExt<R: Runtime> {
  fn keyring(&self) -> &Keyring<R>;
}

impl Keyring<R> {
  pub fn initialize_service(&self, service_name: String) -> Result<()>;
  pub fn set(&self, username: &str, credential_type: CredentialType, value: CredentialValue) -> Result<()>;
  pub fn get(&self, username: &str, credential_type: CredentialType) -> Result<CredentialValue>;
  pub fn delete(&self, username: &str, credential_type: CredentialType) -> Result<()>;
  pub fn exists(&self, username: &str, credential_type: CredentialType) -> Result<bool>;
}
```

## Error Handling

The plugin provides detailed error information for various failure scenarios:

- **Credential not found**: When attempting to retrieve a non-existent credential
- **Access denied**: When the user denies permission to access the keyring
- **Service not available**: When the underlying keyring service is unavailable
- **Invalid input**: When providing invalid parameters
- **Platform errors**: Platform-specific errors from the underlying keyring implementation

In JavaScript:
```typescript
try {
  const password = await getPassword('nonexistent-user')
} catch (error) {
  console.error('Failed to get password:', error.message)
}
```

In Rust:
```rust
match app.keyring().get(&username, CredentialType::Password) {
    Ok(credential) => { /* handle success */ },
    Err(tauri_plugin_keyring::Error::CredentialNotFound(_)) => {
        // Handle missing credential
    },
    Err(e) => {
        // Handle other errors
        eprintln!("Keyring error: {}", e);
    }
}
```

## Security Considerations

1. **Service Name**: Use a unique service name for your application to avoid conflicts with other applications
2. **Credential Isolation**: Each service name creates an isolated credential store
3. **Platform Security**: The plugin leverages platform-native security features:
   - Hardware security modules when available
   - User authentication requirements
   - System-level access controls
4. **Data Protection**: Credentials are encrypted using platform-specific encryption mechanisms

## Examples

A complete example application is available in the `examples/tauri-app` directory, demonstrating:

- Keyring initialization
- Password storage and retrieval
- Binary secret handling
- Error handling
- User interface integration

To run the example:

```bash
cd examples/tauri-app
npm install
npm run tauri dev
```

## Building for Mobile

### Android

The plugin automatically includes necessary Android permissions. No additional setup is required.

### iOS

For iOS deployment, ensure your app has the necessary entitlements for Keychain access in your `Info.plist`:

```xml
<key>keychain-access-groups</key>
<array>
  <string>$(AppIdentifierPrefix)</string>
</array>
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License

Code: (c) 2025 - Present - Charles R. Portwood II

MIT or MIT/Apache 2.0 where applicable.
