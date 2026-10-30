fn hook(game: &crate::game::Game<'_>, enabled: &mut bool) {
    game.rust_sodium_culling(enabled);
}
