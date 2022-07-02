mod number_game;

fn main() {
    let mut ng = number_game::NumberGame::new(1..1000);
    ng.play();
}
