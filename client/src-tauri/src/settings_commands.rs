use ghost_hand_client::config::Config;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::State;

use crate::AppState;

/// User-facing settings subset (serialized to settings.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub server_url: String,
    pub stun_servers: Vec<String>,
    #[serde(default)]
    pub require_auth: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
}

impl AppSettings {
    pub fn from_config(config: &Config) -> Self {
        Self {
            server_url: config.server_url.clone(),
            stun_servers: config.stun_servers.clone(),
            require_auth: config.security_config.require_auth,
            password_hash: config.security_config.password_hash.clone(),
        }
    }

    pub fn apply_to_config(&self, config: &mut Config) {
        config.server_url = self.server_url.clone();
        config.stun_servers = self.stun_servers.clone();
        config.security_config.require_auth = self.require_auth;
        config.security_config.password_hash = self.password_hash.clone();
    }
}

fn settings_path(data_dir: &Path) -> std::path::PathBuf {
    data_dir.join("settings.json")
}

/// Load settings from disk if present, or fall back to Config defaults.
pub fn load_settings_from_disk(data_dir: &Path) -> AppSettings {
    let path = settings_path(data_dir);
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(s) = serde_json::from_str::<AppSettings>(&content) {
            // Basic sanity: URL must be a WebSocket scheme
            if s.server_url.starts_with("ws://") || s.server_url.starts_with("wss://") {
                return s;
            }
        }
    }
    AppSettings::from_config(&Config::default())
}

/// Return the current in-memory settings (server_url + stun_servers).
#[tauri::command]
pub async fn load_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let config = state.config.lock().await;
    Ok(AppSettings::from_config(&config))
}

/// Persist new settings to disk and update the in-memory Config.
/// Returns an error if the server_url scheme is invalid.
/// The caller (App.vue) must call initialize_session() afterwards to reconnect.
#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), String> {
    // Validate WebSocket URL scheme
    if !settings.server_url.starts_with("ws://") && !settings.server_url.starts_with("wss://") {
        return Err(format!(
            "URL invalide '{}': doit commencer par ws:// ou wss://",
            settings.server_url
        ));
    }

    // Validate at least one STUN server
    if settings.stun_servers.is_empty() {
        return Err("Au moins un serveur STUN est requis".to_string());
    }

    // Update in-memory config
    {
        let mut config = state.config.lock().await;
        settings.apply_to_config(&mut config);
    }

    // Persist to settings.json
    let path = settings_path(&state.data_dir);
    let _ = std::fs::create_dir_all(&state.data_dir);
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Erreur sérialisation: {}", e))?;
    std::fs::write(&path, content)
        .map_err(|e| format!("Erreur écriture settings.json: {}", e))?;

    Ok(())
}
