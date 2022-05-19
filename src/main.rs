use typycal::Game;

fn main() -> std::io::Result<()> {
    let game = Game::words(10).num_peek(3);
    game.play()
}
