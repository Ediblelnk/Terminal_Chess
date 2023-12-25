use std::time::Instant;
use terminal_chess as lib;

fn main() {
    let c = lib::Chess::new();
    c.print_board();
    let start = Instant::now();
    let white_moves = c.get_white_moves();
    let duration = start.elapsed();
    println!("White move calculation took: {:?}", duration);
    let start = Instant::now();
    let black_moves = c.get_black_moves();
    let duration = start.elapsed();
    println!("Black move calculation took: {:?}", duration);

    println!("Total white moves; {}", white_moves.len());
    for mv in &white_moves {
        print!(
            "| {:2} -> {:2} |",
            mv.0.checked_ilog2().unwrap_or_default(),
            mv.1.checked_ilog2().unwrap_or_default()
        );
    }
    println!();

    println!("Total black moves; {}", black_moves.len());
    for mv in &black_moves {
        print!(
            "| {:2} -> {:2} |",
            mv.0.checked_ilog2().unwrap_or_default(),
            mv.1.checked_ilog2().unwrap_or_default()
        );
    }
    println!();

    println!("Eval: {}", c.evaluate(&true));
}
