use structopt::StructOpt;
use typycal::Game;

#[derive(Debug, StructOpt)]
struct App {
    #[structopt(default_value = "10")]
    words: usize,
    #[structopt(short, long, default_value = "3")]
    num_peek: u16,
}

fn main() -> std::io::Result<()> {
    let app = App::from_args();
    let game = Game::words(app.words).num_peek(app.num_peek);
    game.play()
}
