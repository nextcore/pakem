use pakem::prelude::Application;
use crate::component::LiveComponent;
use crate::registry::LiveRegistry;

pub trait LiveApplicationExt {
    /// Mengaktifkan runtime reaktif Pakem Live:
    /// - Mendaftarkan direktif Blade: `@kilat`, `@live`, `@livewire`, `@kilatScripts`, `@pakemLive`
    /// - Memasang rute transport WebSocket (`/_pakem/live/ws`), HTTP RPC (`/_pakem/live/call`), dan asset JS
    fn with_live(self) -> Self;

    /// Mendaftarkan komponen reaktif Pakem Live ke registry.
    fn with_live_component<T: LiveComponent>(self, name: &str) -> Self;
}

impl LiveApplicationExt for Application {
    fn with_live(self) -> Self {
        crate::init();
        self.with_raw_router(crate::routes())
    }

    fn with_live_component<T: LiveComponent>(self, name: &str) -> Self {
        crate::init();
        LiveRegistry::global().register::<T>(name);
        self
    }
}
