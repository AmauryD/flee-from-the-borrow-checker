use crate::game::Game;

pub fn render_menu(game: &mut Game, screen_x: u8, screen_y: u8) -> Result<(), std::io::Error> {
    game.screen.set_string_at_with_word_wrap(screen_x + 1, screen_y, "*** 🦀 Crab invasion 🦀 ***".to_string());
    game.screen.set_string_at_with_word_wrap(screen_x + 1, screen_y + 1, "> Use zqsd to move".to_string());
    game.screen.set_string_at_with_word_wrap(screen_x + 1, screen_y + 2, format!("> Level: {}", game.map.level));

    game.screen.set_string_at_with_word_wrap(screen_x + 1, screen_y + 4, "🏆 Scoreboard 🏆".to_string());
    game.screen.set_string_at_with_word_wrap(screen_x + 1, screen_y + 5, format!("> {}: {}", game.player.name, game.player.score));
    Ok(())
}