//! # NUI Module
//!
//! FiveM-compatible NUI system using Tauri WebView.
//!
//! ## Architecture
//!
//! - **Client side**: Tauri WebView renders HTML/CSS/JS
//! - **Server side**: Serves static files, routes messages
//! - **Protocol**: FiveM-compatible `postMessage` system
//!
//! ## Usage
//!
//! ### Register NUI in resource
//!
//! ```lua
//! -- fxmanifest.lua
//! exports['gameverse']:RegisterNUI('myui', 'nui://myui/index.html')
//! ```
//!
//! ### Send message to NUI
//!
//! ```lua
//! TriggerEvent(' SendToUI', {
//!     type = 'showNotification',
//!     data = { message = 'Hello!' }
//! })
//! ```
//!
//! ### Receive message from NUI
//!
//! ```lua
//! RegisterNUICallback('doSomething', function(data)
//!     print('NUI says: ' .. json.encode(data))
//!     return true
//! end)
//! ```

pub mod server;
pub mod api;

pub use server::{NUIServer, NUIMessage, NUIRegistration};
