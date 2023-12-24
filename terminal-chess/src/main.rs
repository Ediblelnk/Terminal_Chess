use terminal_chess as lib;

fn main() {
    let c = lib::Chess::new();
    c.print_board();

    let moves = c.get_knight_moves(
        &0b_00000000_0000000_00000000_00000000_00000000_00000000_00000000_00000010,
    );

    for mv in moves {
        println!("{:X}", mv.1);
    }
}
