import SwiftRs
import Tauri
import UIKit
import WebKit

/**
 * iOS plugin for Tauri Keyring Plugin.
 * 
 * This plugin provides the iOS interface for Tauri's plugin system.
 * All keyring operations are handled by Tauri commands which delegate
 * to the Rust implementation using apple-native-keyring-store for
 * direct iOS Keychain access.
 */
class KeyringPlugin: Plugin {
  // Empty implementation - all functionality is handled by Tauri commands
  // defined in the Rust layer (commands.rs) which use the shared
  // KeyringImplementation with iOS-specific native store
}

@_cdecl("init_plugin_keyring")
public func initPlugin() -> Plugin {
  return KeyringPlugin()
}
