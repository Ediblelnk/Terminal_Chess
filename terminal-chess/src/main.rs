use std::time::Instant;
// use std::{thread, time};
use terminal_chess as lib;

fn main() {
    let mut c = lib::Chess::new();
    c.print_board();
    let mut buf = String::new();
    loop {
        let _ = std::io::stdin().read_line(&mut buf);
        let mut x: (isize, Option<lib::Move>) = (0, None);
        for i in 1..7 {
            let start = Instant::now();
            x = c.minimax(i, isize::MIN + 1, isize::MAX);
            let duration = start.elapsed();
            println!("Time: {:?}", duration);
            println!("{:?}", x);
        }
        c.move_piece(x.1.unwrap());
        c.print_board();
    }
}
