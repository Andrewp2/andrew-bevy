use crate::App;
use anyhow::Result;
use bevy_utils::NonSendBoxedFuture;
use std::any::Any;

/// A collection of Bevy app logic and configuration.
///
/// Plugins configure an [`App`]. When an [`App`] registers a plugin,
/// the plugin's [`Plugin::build`] function is run.
pub trait Plugin: Any + Send + Sync {
    /// Configures the [`App`] to which this plugin is added.
    fn build(&self, app: &mut App);
    /// Configures a name for the [`Plugin`] which is primarily used for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn build_async<'a>(&'a self, app: &'a mut App) -> NonSendBoxedFuture<'a, Result<()>> {
        self.build(app);
        Box::pin(async move { Ok(()) })
    }
}

/// A type representing an unsafe function that returns a mutable pointer to a [`Plugin`].
/// It is used for dynamically loading plugins.
///
/// See `bevy_dynamic_plugin/src/loader.rs#dynamically_load_plugin`.
pub type CreatePlugin = unsafe fn() -> *mut dyn Plugin;
