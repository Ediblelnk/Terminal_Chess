pub mod board {
    pub const RANKS: usize = 8;
    pub const FILES: usize = 8;
    pub const TOP_BORDER: &'static str = r".-.================================================================================================.-.";
    pub const BOTTOM_BORDER: &'static str = r"'-'=========[a]=========[b]=========[c]=========[d]=========[e]=========[f]=========[g]=========[h]'-'";
    pub const LAYERS_PER_RANK: usize = 5;
    pub const MAX_SQUARE_INDEX: usize = 63;

    pub const FILE_A: u64 =
        0b_10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
    pub const FILE_H: u64 =
        0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
    pub const RANK_8: u64 =
        0b_11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    pub const RANK_1: u64 =
        0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
    pub mod index {
        pub mod black {
            pub const PAWN: usize = 0;
            pub const KNIGHT: usize = 1;
            pub const BISHOP: usize = 2;
            pub const ROOK: usize = 3;
            pub const QUEEN: usize = 4;
            pub const KING: usize = 5;
        }
        pub mod white {
            pub const PAWN: usize = 6;
            pub const KNIGHT: usize = 7;
            pub const BISHOP: usize = 8;
            pub const ROOK: usize = 9;
            pub const QUEEN: usize = 10;
            pub const KING: usize = 11;
        }
        pub const EMPTY: usize = 12;
    }
    #[rustfmt::skip]
    pub const PIECE: [[[&'static str; 13]; 5]; 2] = [
    [
        [r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::: _ww_ :::",r"::::\++/::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::: _ww_ :::",r"::::\++/::::",r"::::::::::::"],
        [r"::::::::::::",r"::: __,,::::",r":::::<>:::::",r"::::UUUU::::",r"::: \  / :::",r"::::(  )::::",r"::::::::::::",r"::: __,, :::",r":::::<>:::::",r"::::UUUU::::",r"::: \@@/ :::",r"::::(@@)::::",r"::::::::::::"],
        [r":::::():::::",r":: L   \~ ::",r":::::/\:::::",r"::::|  |::::",r"::: |  | :::",r"::::|  |::::",r":::::():::::",r":: L@@@\~ ::",r":::::/\:::::",r"::: |@@| :::",r"::: |@@| :::",r"::::|@@|::::",r"::::::::::::"],
        [r":::::{}:::::",r":::: ) ( :::",r"::::\  /::::",r"::::|  |::::",r"::: /  \ :::",r"::::/  \::::",r":::: @@ ::::",r":::: )@( :::",r"::: \@@/ :::",r"::: |@@| :::",r"::: /@@\ :::",r"::::/@@\::::",r"::::::::::::"],
        [r"::::{__}::::",r":: {____} ::",r":::{____}:::",r":::{____}:::",r":: {____} ::",r":::{____}:::",r"::: {@@} :::",r":: {@@@@} ::",r":: {@@@@} ::",r":: {@@@@} ::",r":: {@@@@} ::",r":::{@@@@}:::",r"::::::::::::"],
        ],
    [
        [r"            ",r"            ",r"            ",r"            ",r"    _ww_    ",r"    \++/    ",r"            ",r"            ",r"            ",r"            ",r"    _ww_    ",r"    \++/    ",r"            "],
        [r"            ",r"    __,,    ",r"     <>     ",r"    UUUU    ",r"    \  /    ",r"    (  )    ",r"            ",r"    __,,    ",r"     <>     ",r"    UUUU    ",r"    \@@/    ",r"    (@@)    ",r"            "],
        [r"     ()     ",r"   L   \~   ",r"     /\     ",r"    |  |    ",r"    |  |    ",r"    |  |    ",r"     ()     ",r"   L@@@\~   ",r"     /\     ",r"    |@@|    ",r"    |@@|    ",r"    |@@|    ",r"            "],
        [r"     {}     ",r"     ) (    ",r"    \  /    ",r"    |  |    ",r"    /  \    ",r"    /  \    ",r"     @@     ",r"     )@(    ",r"    \@@/    ",r"    |@@|    ",r"    /@@\    ",r"    /@@\    ",r"            "],
        [r"    {__}    ",r"   {____}   ",r"   {____}   ",r"   {____}   ",r"   {____}   ",r"   {____}   ",r"    {@@}    ",r"   {@@@@}   ",r"   {@@@@}   ",r"   {@@@@}   ",r"   {@@@@}   ",r"   {@@@@}   ",r"            "],
    ],
];
}

fn rotate_left(n: &u64, m: &u32) -> u64 {
    n.rotate_left(*m)
}

fn rotate_right(n: &u64, m: &u32) -> u64 {
    n.rotate_right(*m)
}

/**
 * returns if the select piece is on the 'a' file
 */
fn is_on_file_a(bit_piece: &u64) -> bool {
    bit_piece & board::FILE_A > 0
}

/**
 * returns if the select piece is on the 'h' file
 */
fn is_on_file_h(bit_piece: &u64) -> bool {
    bit_piece & board::FILE_H > 0
}

/**
 * returns if the select piece is on the '8' rank
 */
fn is_on_rank_8(bit_piece: &u64) -> bool {
    bit_piece & board::RANK_8 > 0
}

/**
 * returns if the select piece is on the '1' rank
 */
fn is_on_rank_1(bit_piece: &u64) -> bool {
    bit_piece & board::RANK_1 > 0
}

/**
 * Each board is stored as bitmaps of the pieces, since a board is 8x8 we use a u64
 * The bit position corresponds to a place on the board:
 *    -------------------------
 * 8 | 63 62 61 60 59 58 57 56 |
 * 7 | 55 54 53 52 51 50 49 48 |
 * 6 | 47 46 45 44 43 42 41 40 |
 * 5 | 39 38 37 36 35 34 33 32 |
 * 4 | 31 30 29 28 27 26 25 24 |
 * 3 | 23 22 21 20 19 18 17 16 |
 * 2 | 15 14 13 12 11 10 09 08 |
 * 1 | 07 06 05 04 03 02 01 00 |
 *    -------------------------
 *      a  b  c  d  e  f  g  h
 */
pub struct Chess {
    bit_boards: [u64; board::index::EMPTY],
}

#[derive(Debug)]
struct NotFound;

impl Chess {
    pub fn new() -> Self {
        Self {
            bit_boards: [
                0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000, // black pawns
                0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // black knights
                0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // black bishops
                0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // black rooks
                0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // black queens
                0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // black kings
                0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000, // white pawns
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010, // white knights
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100, // white bishops
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001, // white rooks
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000, // white queens
                0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000, // white kings
            ],
        }
    }

    /**
     * returns the move pairs for the bit_piece when moved as a rook
     */
    pub fn get_cartesian_moves(self: &Self, bit_piece: u64) -> Vec<(u64, u64)> {
        let white_bits = self.get_white_bit_board();
        let black_bits = self.get_black_bit_board();
        let is_white = { bit_piece & white_bits > 0 };
        let mut moves = Vec::<(u64, u64)>::new();

        moves.append(&mut Self::get_marching_moves(
            rotate_left,
            1,
            is_on_file_h,
            is_white,
            &bit_piece,
            &white_bits,
            &black_bits,
        ));
        moves.append(&mut Self::get_marching_moves(
            rotate_right,
            1,
            is_on_file_a,
            is_white,
            &bit_piece,
            &white_bits,
            &black_bits,
        ));
        moves.append(&mut Self::get_marching_moves(
            rotate_left,
            8,
            is_on_rank_8,
            is_white,
            &bit_piece,
            &white_bits,
            &black_bits,
        ));
        moves.append(&mut Self::get_marching_moves(
            rotate_right,
            8,
            is_on_rank_1,
            is_white,
            &bit_piece,
            &white_bits,
            &black_bits,
        ));

        moves
    }

    /**
     * approves a move if black capturing white, white capturing black, or either capturing nothing.
     * the first bool is approval of move, second bool is whether it ends the search or not
     */
    fn approve_move(
        is_white: bool,
        test_move: &u64,
        white_bits: &u64,
        black_bits: &u64,
    ) -> (bool, bool) {
        // either capturing nothing
        if test_move & (white_bits | black_bits) == 0 {
            return (true, true);
        }
        // capturing white, true if black, meaning not white
        if test_move & white_bits > 0 {
            return (!is_white, false);
        // capturing black, true if white
        } else {
            return (is_white, false);
        }
    }

    fn get_marching_moves(
        shifting_fn: fn(&u64, &u32) -> u64,
        shifting_arg: u32,
        break_fn: fn(&u64) -> bool,
        is_white: bool,
        bit_piece: &u64,
        white_bits: &u64,
        black_bits: &u64,
    ) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();
        let mut test_move = bit_piece.clone();

        loop {
            test_move = shifting_fn(&test_move, &shifting_arg);

            // the piece looped to the row on the top
            if break_fn(&test_move) {
                break;
            }

            match Self::approve_move(is_white, &test_move, &white_bits, &black_bits) {
                (true, true) => moves.push((*bit_piece, test_move)),
                (true, false) => {
                    moves.push((*bit_piece, test_move));
                    break;
                }
                (false, _) => break,
            }
        }

        moves
    }

    /**
     * gets the bit board of all the white pieces
     */
    fn get_white_bit_board(self: &Self) -> u64 {
        self.bit_boards[board::index::white::PAWN]
            | self.bit_boards[board::index::white::KNIGHT]
            | self.bit_boards[board::index::white::BISHOP]
            | self.bit_boards[board::index::white::ROOK]
            | self.bit_boards[board::index::white::QUEEN]
            | self.bit_boards[board::index::white::KING]
    }

    /**
     * gets the bit board of all the black pieces
     */
    fn get_black_bit_board(self: &Self) -> u64 {
        self.bit_boards[board::index::black::PAWN]
            | self.bit_boards[board::index::black::KNIGHT]
            | self.bit_boards[board::index::black::BISHOP]
            | self.bit_boards[board::index::black::ROOK]
            | self.bit_boards[board::index::black::QUEEN]
            | self.bit_boards[board::index::black::KING]
    }

    /**
     * moves a piece based on its starting and ending position, making sure to remove any pieces it captures
     */
    pub fn move_piece(self: &mut Self, bit_move_start: u64, bit_mov_end: u64) {
        self.bit_boards = self.bit_boards.map(|bit_board| bit_board & !bit_mov_end);

        let index_of_moved = self.get_index_moved(bit_move_start).unwrap();
        self.bit_boards[index_of_moved] ^= bit_move_start | bit_mov_end;
    }

    /**
     * Gets the index of the bit_board from which a piece is moving.
     */
    fn get_index_moved(self: &Self, bit_move_start: u64) -> Result<usize, NotFound> {
        for (j, bit_board) in self.bit_boards.iter().enumerate() {
            if bit_board & bit_move_start > 0 {
                return Ok(j);
            }
        }

        Err(NotFound)
    }

    pub fn print_board(self: &Self) {
        let squares = self.get_square_array();

        println!("{}", board::TOP_BORDER);
        for rank in 0..board::RANKS {
            for layer in 0..board::LAYERS_PER_RANK {
                if layer == 0 {
                    print!("[{}]", 8 - rank);
                } else {
                    print!("| |");
                }
                for file in 0..board::FILES {
                    print!(
                        "{}",
                        board::PIECE[(rank + file) % 2][layer]
                            [squares[board::MAX_SQUARE_INDEX - (rank * board::FILES + file)]]
                    );
                }
                println!("| |");
            }
        }
        println!("{}", board::BOTTOM_BORDER);
    }

    /**
     * Returns an array of indexes, used in printing out the board
     */
    fn get_square_array(self: &Self) -> [usize; 64] {
        let mut board = [board::index::EMPTY; 64];
        let mut place: u64 =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;

        // goes through each of the bits and determines what piece is there
        for i in 0..u64::BITS {
            for (j, bit_board) in self.bit_boards.iter().enumerate() {
                if bit_board & place > 0 {
                    board[i as usize] = j;
                    break;
                }
            }
            place <<= 1;
        }

        board
    }
}

pub fn sample_board() -> &'static str {
    r"
.-.================================================================================================.-.
[8]::::::::::::            ::::::::::::    _ww_    ::::\++/::::            ::::::::::::            | |
| |::::UUUU::::    __,,    :::::<>:::::    \  /    ::::(  )::::     <>     ::: __,,::::    UUUU    | |
| |::::|  |::::   L   \~   :::::/\:::::    |  |    ::::|  |::::     /\     :: L   \~ ::    |  |    | |
| |::::|  |::::     ) (    ::::\  /::::    /  \    ::::/  \::::    \  /    :::: ) ( :::    |  |    | |
| |:::{____}:::   {____}   :::{____}:::   {____}   :::{____}:::   {____}   :: {____} ::   {____}   | |
[7]            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |     ()     :::::():::::     ()     :::::():::::     ()     :::::():::::     ()     :::::():::::| |
| |     {}     :::::{}:::::     {}     :::::{}:::::     {}     :::::{}:::::     {}     :::::{}:::::| |
| |    {__}    ::::{__}::::    {__}    ::::{__}::::    {__}    ::::{__}::::    {__}    ::::{__}::::| |
[6]::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
[5]            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
[4]::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
[3]            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
[2]::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |:::: () ::::     ()     :::: () ::::     ()     :::: () ::::     ()     :::: () ::::     ()     | |
| |:::: @@ ::::     @@     :::: @@ ::::     @@     :::: @@ ::::     @@     :::: @@ ::::     @@     | |
| |::: {@@} :::    {@@}    ::: {@@} :::    {@@}    ::: {@@} :::    {@@}    ::: {@@} :::    {@@}    | |
[1]            ::::::::::::            ::: _ww_ :::    \++/    ::::::::::::            ::::::::::::| |
| |    UUUU    ::: __,, :::     <>     ::: \@@/ :::    (@@)    :::: <> ::::    __,,    ::: UUUU :::| |
| |    |@@|    :: L@@@\~ ::     /\     ::: |@@| :::    |@@|    :::: /\ ::::   L@@@\~   ::: |@@| :::| |
| |    |@@|    :::: )@( :::    \@@/    ::: /@@\ :::    /@@\    ::: \@@/ :::     )@(    ::: |@@| :::| |
| |   {@@@@}   :: {@@@@} ::   {@@@@}   :: {@@@@} ::   {@@@@}   :: {@@@@} ::   {@@@@}   :: {@@@@} ::| |
'-'=========[a]=========[b]=========[c]=========[d]=========[e]=========[f]=========[g]=========[h]'-'"
}
