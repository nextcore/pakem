//! Re-export engine Blade dari crate mandiri `pakem-blade` dengan integrasi runtime Pakem.

pub use pakem_blade::*;

use std::sync::Arc;

/// Ekstensi builder untuk BladeEngine di ekosistem Pakem Framework
pub trait PakemBladeEngineExt {
    fn with_pakem_sandbox(self, manager: Arc<crate::core::sandbox::PakemPluginManager>) -> Self;
}

impl PakemBladeEngineExt for BladeEngine {
    fn with_pakem_sandbox(self, manager: Arc<crate::core::sandbox::PakemPluginManager>) -> Self {
        self.with_plugin_runner(manager)
    }
}
