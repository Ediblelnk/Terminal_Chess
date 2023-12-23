use terminal_chess as lib;

fn main() {
    let c = lib::Chess::new();
    c.print_board();
    println!(
        "{:?}",
        c.get_white_pawn_moves(
            0b_00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000,
        )
    );
}
