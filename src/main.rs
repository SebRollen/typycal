use typycal::Game;

fn main() -> std::io::Result<()> {
    let game = Game::words(10);
    game.play()
}
