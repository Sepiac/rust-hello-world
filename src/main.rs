mod number_game;

fn main() {
    let mut ng = number_game::NumberGame::new(10..20);
    ng.play();
}
