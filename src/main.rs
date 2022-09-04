use rist::boards::classic_board::new;

fn main() {
    let classic_board = new(4);
    println! {"{:#?}", classic_board};
}
