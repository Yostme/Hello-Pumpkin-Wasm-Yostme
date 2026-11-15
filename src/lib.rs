use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::*;

mod listeners;
mod commands;

struct EssentialsPlugin;
impl Plugin for EssentialsPlugin {
    fn new() -> Self {
        Self
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "essentials".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Yostme".into()],
            description: "A plugin with essential commands built for Pumpkin.".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        // plugin load message
        info!("Essentials plugin successfully loaded!");
        // start listeners
        listeners::init(&context)?;
        // start commands
        commands::init(&context)?;
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        // plugin unload message
        info!("Essentials plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(EssentialsPlugin);
