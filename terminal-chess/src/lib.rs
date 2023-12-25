pub mod board {
    pub const RANKS: usize = 8;
    pub const FILES: usize = 8;
    pub const TOP_BORDER: &'static str = r".-.================================================================================================.-.";
    pub const BOTTOM_BORDER: &'static str = r"'-'=========[a]=========[b]=========[c]=========[d]=========[e]=========[f]=========[g]=========[h]'-'";
    pub const LAYERS_PER_RANK: usize = 5;
    pub const MAX_SQUARE_INDEX: usize = 63;

    pub const FILE_A: u64 =
        0b_10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
    pub const FILE_B: u64 =
        0b_01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
    pub const FILE_G: u64 =
        0b_00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
    pub const FILE_H: u64 =
        0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
    pub const RANK_8: u64 =
        0b_11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    pub const RANK_7: u64 =
        0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
    pub const RANK_2: u64 =
        0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
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

pub mod bit_math {
    use crate::board;

    pub fn is_file_a(bit_piece: &u64) -> bool {
        bit_piece & board::FILE_A > 0
    }

    pub fn is_file_b(bit_piece: &u64) -> bool {
        bit_piece & board::FILE_B > 0
    }

    pub fn is_file_g(bit_piece: &u64) -> bool {
        bit_piece & board::FILE_G > 0
    }

    pub fn is_file_h(bit_piece: &u64) -> bool {
        bit_piece & board::FILE_H > 0
    }

    pub fn is_rank_8(bit_piece: &u64) -> bool {
        bit_piece & board::RANK_8 > 0
    }

    pub fn is_rank_7(bit_piece: &u64) -> bool {
        bit_piece & board::RANK_7 > 0
    }

    pub fn is_rank_2(bit_piece: &u64) -> bool {
        bit_piece & board::RANK_2 > 0
    }

    pub fn is_rank_1(bit_piece: &u64) -> bool {
        bit_piece & board::RANK_1 > 0
    }
}

/**
 * Each board is stored as bitmaps of the pieces, since a board is 8x8 we use a u64
 * The bit position corresponds to a place on the board:
 *   `-------------------------`
 * 8 | 63 62 61 60 59 58 57 56 |
 * 7 | 55 54 53 52 51 50 49 48 |
 * 6 | 47 46 45 44 43 42 41 40 |
 * 5 | 39 38 37 36 35 34 33 32 |
 * 4 | 31 30 29 28 27 26 25 24 |
 * 3 | 23 22 21 20 19 18 17 16 |
 * 2 | 15 14 13 12 11 10 09 08 |
 * 1 | 07 06 05 04 03 02 01 00 |
 *   `-------------------------`
 *      a  b  c  d  e  f  g  h
 */

#[derive(Clone, Copy)]
pub struct Chess {
    bit_boards: [u64; board::index::EMPTY],
    white_bits: u64,
    black_bits: u64,
}

#[derive(Debug)]
struct NotFound;

impl Chess {
    /**
     * sets up the bit board for a default game of chess
     */
    pub fn new() -> Self {
        let mut s = Self {
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
            white_bits: 0,
            black_bits: 0,
        };

        s.update_black_bits().update_white_bits();

        s
    }

    pub fn get_white_moves(self: &Self) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();

        let mut bit_piece = 1u64;
        for _ in 0..u64::BITS {
            // if there is no white piece here, just move on
            if bit_piece & self.white_bits == 0 {
                bit_piece <<= 1;
                continue;
            }

            if bit_piece & self.bit_boards[board::index::white::PAWN] > 0 {
                moves.append(&mut self.get_white_pawn_moves(&bit_piece));
            } else if bit_piece & self.bit_boards[board::index::white::KNIGHT] > 0 {
                moves.append(&mut self.get_knight_moves(&bit_piece, &true));
            } else if bit_piece & self.bit_boards[board::index::white::BISHOP] > 0 {
                moves.append(&mut self.get_diagonal_moves(&bit_piece, &true));
            } else if bit_piece & self.bit_boards[board::index::white::ROOK] > 0 {
                moves.append(&mut self.get_cartesian_moves(&bit_piece, &true));
            } else if bit_piece & self.bit_boards[board::index::white::QUEEN] > 0 {
                moves.append(&mut self.get_cartesian_moves(&bit_piece, &true));
                moves.append(&mut self.get_diagonal_moves(&bit_piece, &true));
            } else {
                moves.append(&mut self.get_king_moves(&bit_piece, &true));
            }

            bit_piece <<= 1;
        }

        moves
    }

    pub fn get_black_moves(self: &Self) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();

        let mut bit_piece = 1u64;
        for _ in 0..u64::BITS {
            // if there is no white piece here, just move on
            if bit_piece & self.black_bits == 0 {
                bit_piece <<= 1;
                continue;
            }

            if bit_piece & self.bit_boards[board::index::black::PAWN] > 0 {
                moves.append(&mut self.get_black_pawn_moves(&bit_piece));
            } else if bit_piece & self.bit_boards[board::index::black::KNIGHT] > 0 {
                moves.append(&mut self.get_knight_moves(&bit_piece, &false));
            } else if bit_piece & self.bit_boards[board::index::black::BISHOP] > 0 {
                moves.append(&mut self.get_diagonal_moves(&bit_piece, &false));
            } else if bit_piece & self.bit_boards[board::index::black::ROOK] > 0 {
                moves.append(&mut self.get_cartesian_moves(&bit_piece, &false));
            } else if bit_piece & self.bit_boards[board::index::black::QUEEN] > 0 {
                moves.append(&mut self.get_cartesian_moves(&bit_piece, &false));
                moves.append(&mut self.get_diagonal_moves(&bit_piece, &false));
            } else {
                moves.append(&mut self.get_king_moves(&bit_piece, &false));
            }

            bit_piece <<= 1;
        }

        moves
    }

    /**
     * returns the move pairs for the bit_piece when moved as a rook
     */
    fn get_cartesian_moves(self: &Self, bit_piece: &u64, is_white: &bool) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();

        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_left(1),
            bit_math::is_file_h,
            &is_white,
            &bit_piece,
        ));
        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_right(1),
            bit_math::is_file_a,
            &is_white,
            &bit_piece,
        ));
        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_left(8),
            bit_math::is_rank_1,
            &is_white,
            &bit_piece,
        ));
        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_right(8),
            bit_math::is_rank_8,
            &is_white,
            &bit_piece,
        ));

        moves
    }

    /**
     * returns the move pairs for the bit_piece when moved as a bishop
     */
    fn get_diagonal_moves(self: &Self, bit_piece: &u64, is_white: &bool) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();

        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_left(9),
            |x| bit_math::is_rank_1(x) || bit_math::is_file_h(x),
            &is_white,
            &bit_piece,
        ));
        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_left(7),
            |x| bit_math::is_rank_1(x) || bit_math::is_file_a(x),
            &is_white,
            &bit_piece,
        ));
        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_right(9),
            |x| bit_math::is_rank_8(x) || bit_math::is_file_a(x),
            &is_white,
            &bit_piece,
        ));
        moves.append(&mut self.get_marching_moves(
            |x| x.rotate_right(7),
            |x| bit_math::is_rank_8(x) || bit_math::is_file_h(x),
            &is_white,
            &bit_piece,
        ));

        moves
    }

    fn get_black_pawn_moves(self: &Self, bit_piece: &u64) -> Vec<(u64, u64)> {
        let all_bits = self.white_bits | self.black_bits;
        let mut moves = Vec::<(u64, u64)>::new();

        // strictly moving move(s)
        let move_1 = bit_piece.rotate_right(8);
        if move_1 & all_bits == 0 {
            moves.push((*bit_piece, move_1));

            if bit_math::is_rank_7(&bit_piece) {
                let move_2 = bit_piece.rotate_right(16);
                if move_2 & all_bits == 0 {
                    moves.push((*bit_piece, move_2));
                }
            }
        }

        // strictly attacking moves
        // left attack
        if !bit_math::is_file_a(&bit_piece) {
            let left_attack = bit_piece.rotate_right(7);
            if left_attack & self.white_bits > 0 {
                moves.push((*bit_piece, left_attack));
            }
        }
        // right attack
        if !bit_math::is_file_h(&bit_piece) {
            let right_attack = bit_piece.rotate_right(9);
            if right_attack & self.white_bits > 0 {
                moves.push((*bit_piece, right_attack));
            }
        }

        moves
    }

    fn get_white_pawn_moves(self: &Self, bit_piece: &u64) -> Vec<(u64, u64)> {
        let all_bits = self.white_bits | self.black_bits;
        let mut moves = Vec::<(u64, u64)>::new();

        // strictly moving move(s)
        let move_1 = bit_piece.rotate_left(8);
        if move_1 & all_bits == 0 {
            moves.push((*bit_piece, move_1));

            if bit_math::is_rank_2(&bit_piece) {
                let move_2 = bit_piece.rotate_left(16);
                if move_2 & all_bits == 0 {
                    moves.push((*bit_piece, move_2));
                }
            }
        }

        // strictly attacking moves
        // left attack
        if !bit_math::is_file_a(&bit_piece) {
            let left_attack = bit_piece.rotate_left(9);
            if left_attack & self.black_bits > 0 {
                moves.push((*bit_piece, left_attack));
            }
        }
        // right attack
        if !bit_math::is_file_h(&bit_piece) {
            let right_attack = bit_piece.rotate_left(7);
            if right_attack & self.black_bits > 0 {
                moves.push((*bit_piece, right_attack));
            }
        }

        moves
    }

    fn get_knight_moves(self: &Self, bit_piece: &u64, is_white: &bool) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();

        // precompute files
        let file_a = bit_math::is_file_a(bit_piece);
        let file_b = bit_math::is_file_b(bit_piece);
        let file_g = bit_math::is_file_g(bit_piece);
        let file_h = bit_math::is_file_h(bit_piece);

        // precompute ranks
        let rank_1 = bit_math::is_rank_1(bit_piece);
        let rank_2 = bit_math::is_rank_2(bit_piece);
        let rank_7 = bit_math::is_rank_7(bit_piece);
        let rank_8 = bit_math::is_rank_8(bit_piece);

        /* .  .  . 15  .
         * .  .  .  .  .
         * .  .  N  .  .
         * .  .  .  .  .
         * .  .  .  .  .
         */
        if !rank_7 && !rank_8 && !file_h {
            moves.append(&mut self.get_move(|x| x << 15u64, &is_white, &bit_piece));
        }

        /* .  .  .  x  .
         * .  .  .  .  6
         * .  .  N  .  .
         * .  .  .  .  .
         * .  .  .  .  .
         */
        if !rank_8 && !file_g && !file_h {
            moves.append(&mut self.get_move(|x| x << 6u64, &is_white, &bit_piece));
        }

        /* .  .  .  x  .
         * .  .  .  .  x
         * .  .  N  .  .
         * .  .  .  . 10
         * .  .  .  .  .
         */
        if !rank_1 && !file_g && !file_h {
            moves.append(&mut self.get_move(|x| x >> 10u64, &is_white, &bit_piece));
        }

        /* .  .  .  x  .
         * .  .  .  .  x
         * .  .  N  .  .
         * .  .  .  .  x
         * .  .  . 17  .
         */
        if !rank_1 && !rank_2 && !file_h {
            moves.append(&mut self.get_move(|x| x >> 17u64, &is_white, &bit_piece));
        }

        /* .  .  .  x  .
         * .  .  .  .  x
         * .  .  N  .  .
         * .  .  .  .  x
         * . 15  .  x  .
         */
        if !rank_1 && !rank_2 && !file_a {
            moves.append(&mut self.get_move(|x| x >> 15u64, &is_white, &bit_piece));
        }

        /* .  .  .  x  .
         * .  .  .  .  x
         * .  .  N  .  .
         * 6  .  .  .  x
         * .  x  .  x  .
         */
        if !rank_1 && !file_a && !file_b {
            moves.append(&mut self.get_move(|x| x >> 6u64, &is_white, &bit_piece));
        }

        /* .  .  .  x  .
         *10  .  .  .  x
         * .  .  N  .  .
         * x  .  .  .  x
         * .  x  .  x  .
         */
        if !rank_8 && !file_a && !file_b {
            moves.append(&mut self.get_move(|x| x << 10u64, &is_white, &bit_piece));
        }

        /* . 17  .  x  .
         * x  .  .  .  x
         * .  .  N  .  .
         * x  .  .  .  x
         * .  x  .  x  .
         */
        if !rank_7 && !rank_8 && !file_a {
            moves.append(&mut self.get_move(|x| x << 17u64, &is_white, &bit_piece));
        }

        moves
    }

    fn get_king_moves(self: &Self, bit_piece: &u64, is_white: &bool) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();

        let file_a = bit_math::is_file_a(bit_piece);
        let file_h = bit_math::is_file_h(bit_piece);

        let rank_1 = bit_math::is_rank_1(bit_piece);
        let rank_8 = bit_math::is_rank_8(bit_piece);

        /* .  .  .
         * .  K  1
         * .  .  .
         */
        if !file_h {
            moves.append(&mut self.get_move(|x| x >> 1, &is_white, bit_piece));
        }

        /* .  .  .
         * .  K  x
         * .  .  9
         */
        if !rank_1 && !file_h {
            moves.append(&mut self.get_move(|x| x >> 9, &is_white, bit_piece));
        }

        /* .  .  .
         * .  K  x
         * .  8  x
         */
        if !rank_1 {
            moves.append(&mut self.get_move(|x| x >> 8, &is_white, bit_piece));
        }

        /* .  .  .
         * .  K  x
         * 7  x  x
         */
        if !rank_1 && !file_a {
            moves.append(&mut self.get_move(|x| x >> 7, &is_white, bit_piece));
        }

        /* .  .  .
         * 1  K  x
         * x  x  x
         */
        if !file_a {
            moves.append(&mut self.get_move(|x| x << 1, &is_white, bit_piece));
        }

        /* 9  .  .
         * x  K  x
         * x  x  x
         */
        if !rank_8 && !file_a {
            moves.append(&mut self.get_move(|x| x << 9, &is_white, bit_piece));
        }

        /* x  8  .
         * x  K  x
         * x  x  x
         */
        if !rank_8 && !file_a {
            moves.append(&mut self.get_move(|x| x << 8, &is_white, bit_piece));
        }

        /* x  x  7
         * x  K  x
         * x  x  x
         */
        if !rank_8 && !file_h {
            moves.append(&mut self.get_move(|x| x << 7, &is_white, bit_piece));
        }

        moves
    }

    /**
     * approves a move if black capturing white, white capturing black, or either capturing nothing.
     * the first bool is approval of move, second bool is whether it ends the search or not
     */
    fn approve_move(self: &Self, is_white: &bool, test_move: &u64) -> (bool, bool) {
        // either capturing nothing
        if test_move & (self.white_bits | self.black_bits) == 0 {
            return (true, true);
        }
        // capturing white, true if black, meaning not white
        if test_move & self.white_bits > 0 {
            return (!is_white, false);
        // capturing black, true if white
        } else {
            return (*is_white, false);
        }
    }

    /**
     * returns the moves that are valid when marching according to the specified bit rotate amount
     */
    fn get_marching_moves(
        self: &Self,
        shifting_fn: fn(&u64) -> u64,
        break_fn: fn(&u64) -> bool,
        is_white: &bool,
        bit_piece: &u64,
    ) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();
        let mut test_move = bit_piece.clone();

        loop {
            test_move = shifting_fn(&test_move);

            // the piece looped to the row on the top
            if break_fn(&test_move) {
                break;
            }

            match self.approve_move(&is_white, &test_move) {
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
     * returns a moves if it is valid according to the specified bit rotate amount
     */
    fn get_move(
        self: &Self,
        shifting_fn: fn(&u64) -> u64,
        is_white: &bool,
        bit_piece: &u64,
    ) -> Vec<(u64, u64)> {
        let mut moves = Vec::<(u64, u64)>::new();
        let test_move = shifting_fn(&bit_piece);

        if self.approve_move(&is_white, &test_move).0 {
            moves.push((*bit_piece, test_move))
        }

        moves
    }

    /**
     * gets the bit board of all the white pieces
     */
    fn update_white_bits(self: &mut Self) -> &mut Self {
        self.white_bits = self.bit_boards[board::index::white::PAWN]
            | self.bit_boards[board::index::white::KNIGHT]
            | self.bit_boards[board::index::white::BISHOP]
            | self.bit_boards[board::index::white::ROOK]
            | self.bit_boards[board::index::white::QUEEN]
            | self.bit_boards[board::index::white::KING];

        self
    }

    /**
     * gets the bit board of all the black pieces
     */
    fn update_black_bits(self: &mut Self) -> &mut Self {
        self.black_bits = self.bit_boards[board::index::black::PAWN]
            | self.bit_boards[board::index::black::KNIGHT]
            | self.bit_boards[board::index::black::BISHOP]
            | self.bit_boards[board::index::black::ROOK]
            | self.bit_boards[board::index::black::QUEEN]
            | self.bit_boards[board::index::black::KING];

        self
    }

    /**
     * moves a piece based on its starting and ending position, making sure to remove any pieces it captures
     */
    pub fn move_piece(self: &mut Self, (bit_move_start, bit_mov_end): (u64, u64)) -> &mut Self {
        self.bit_boards = self.bit_boards.map(|bit_board| bit_board & !bit_mov_end);
        self.bit_boards[self.get_index_moved(bit_move_start).unwrap()] ^=
            bit_move_start | bit_mov_end;
        self.update_black_bits().update_white_bits()
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

    pub fn print_board(self: &Self) -> &Self {
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

        self
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
