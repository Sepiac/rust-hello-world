mod number_game;

fn main() {
    let mut ng = number_game::NumberGame {
        ..Default::default()
    };
    ng.play();
}
