use super::rust_sodium_core::Budget;
use super::{Game, native_hooks::NativeMod};
use std::cell::Cell;
thread_local! {static STATE:Cell<Budget>=Cell::new(Budget::default());}
const ID: &str = "rust_sodium";
impl Game<'_> {
    pub(super) fn rust_sodium_frame(&mut self, elapsed: std::time::Duration) {
        if !self.native_window() || !self.native_on(ID) || !self.native_playing() || !self.focused {
            return;
        }
        STATE.with(|s| {
            let mut b = s.get();
            b.sample(elapsed.as_secs_f32() * 1000.0);
            s.set(b);
        });
    }
    pub(super) fn rust_sodium_graphics(&self) -> horizon_client::graphics::Graphics {
        let mut g = self.settings.horizon.clone();
        if self.native_on(ID) {
            STATE.with(|s| {
                let b = s.get();
                g.chunk_updates = b.chunks;
                g.upload_budget_kib = b.kib;
            });
        }
        g
    }
    pub(super) fn rust_sodium_list(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod{id:ID.into(),title:"Sodium-inspired Rust 0.1.0".into(),description:"Uploads adaptativos e oclusão. Implementação independente; ganhos dependem do hardware.".into(),keys:vec![]});
    }
}
