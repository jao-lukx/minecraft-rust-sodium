fn hook(game: &mut crate::game::Game<'_>, mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    game.rust_sodium_list(mods);
}
