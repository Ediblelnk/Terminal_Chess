use std::time::Instant;
// use std::{thread, time};
use terminal_chess as lib;

fn main() {
    let mut c = lib::Chess::new();
    c.print_board();
    let mut buf = String::new();
    loop {
        let _ = std::io::stdin().read_line(&mut buf);

        let start = Instant::now();
        let x = c.minimax(5, isize::MIN + 2, isize::MAX - 1);
        let duration = start.elapsed();

        println!("Time: {:?}", duration);
        println!("{:?}\n", x);
        println!("Eval: {}", c.evaluate());
        c.move_piece(x.1.unwrap());
        println!("Legal Moves: {:?}", c.get_legal_moves());
        c.print_board();
    }
}
