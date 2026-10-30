use super::rust_sodium_core::Budget;
use super::{Game, native_hooks::NativeMod};
use std::cell::Cell;
thread_local! {static BUDGET:Cell<Budget>=Cell::new(Budget::default());}
const ID: &str = "rust_sodium";
impl Game<'_> {
    pub(super) fn rust_sodium_frame(&mut self, elapsed: std::time::Duration) {
        if self.native_window() && self.native_on(ID) && self.native_playing() && self.focused {
            BUDGET.with(|s| {
                let mut b = s.get();
                b.sample(elapsed.as_secs_f32() * 1000.0);
                s.set(b);
            });
        }
    }
    pub(super) fn rust_sodium_budget(&self, budget: &mut Option<(u32, u32)>) {
        if self.native_on(ID) {
            BUDGET.with(|s| {
                let b = s.get();
                *budget = Some((b.chunks, b.kib));
            });
        }
    }
    pub(super) fn rust_sodium_culling(&self, enabled: &mut bool) {
        *enabled |= self.native_on(ID);
    }
    pub(super) fn rust_sodium_list(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod {
            id: ID.into(),
            title: "Sodium-inspired Rust 0.2.0".into(),
            description: "Uploads adaptativos e oclusão. Não é um port completo do Sodium.".into(),
            keys: vec![],
        });
    }
}
