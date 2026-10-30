fn hook(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    game.rust_sodium_frame(elapsed);
}
