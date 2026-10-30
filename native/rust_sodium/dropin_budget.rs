fn hook(game: &crate::game::Game<'_>, budget: &mut Option<(u32, u32)>) {
    game.rust_sodium_budget(budget);
}
