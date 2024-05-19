use rand::Rng;

pub struct Board {
    pub player_board: u64,
    pub opponent_board: u64,
}

pub fn make_bitboard(board: Vec<bool>) -> u64 {
    let mut bitboard = 0;
    for i in 0..64 {
        if board[i] {
            bitboard |= 1 << i;
        }
    }
    bitboard
}

pub fn make_normal_board(bitboard: u64) -> Vec<bool> {
    let mut board = vec![false; 64];
    for i in 0..64 {
        if bitboard & (1 << i) != 0 {
            board[i] = true;
        }
    }
    board
}

impl Board {
    pub fn new(player_board: Vec<bool>, opponent_board: Vec<bool>) -> Self {
        let player_board = make_bitboard(player_board);
        let opponent_board = make_bitboard(opponent_board);

        Board {
            player_board,
            opponent_board,
        }
    }

    pub fn coordinates_to_bit(&self, x: u32, y: u32) -> u64 {
        1 << (x + 8 * y)
    }

    pub fn can_put(&self, put: u64) -> bool {
        let legal_board = self.get_legal_board();
        legal_board & put == put
    }

    pub fn get_legal_board(&self) -> u64 {
        let horizontal_watch_board = self.opponent_board & 0x7e7e7e7e7e7e7e7e;
        let vertical_watch_board = self.opponent_board & 0x00ffffffffffff00;
        let all_side_watch_board = self.opponent_board & 0x007e7e7e7e7e7e00;
        let blank_board = !(self.player_board | self.opponent_board);
        let mut tmp = 0;
        let mut legal_board = 0;

        // left
        tmp = horizontal_watch_board & (self.player_board << 1);
        for _ in 0..5 {
            tmp |= horizontal_watch_board & (tmp << 1);
        }
        legal_board |= blank_board & (tmp << 1);

        // right
        tmp = horizontal_watch_board & (self.player_board >> 1);
        for _ in 0..5 {
            tmp |= horizontal_watch_board & (tmp >> 1);
        }
        legal_board |= blank_board & (tmp >> 1);

        // up
        tmp = vertical_watch_board & (self.player_board << 8);
        for _ in 0..5 {
            tmp |= vertical_watch_board & (tmp << 8);
        }
        legal_board |= blank_board & (tmp << 8);

        // down
        tmp = vertical_watch_board & (self.player_board >> 8);
        for _ in 0..5 {
            tmp |= vertical_watch_board & (tmp >> 8);
        }
        legal_board |= blank_board & (tmp >> 8);

        // upper left
        tmp = all_side_watch_board & (self.player_board << 9);
        for _ in 0..5 {
            tmp |= all_side_watch_board & (tmp << 9);
        }
        legal_board |= blank_board & (tmp << 9);

        // upper right
        tmp = all_side_watch_board & (self.player_board << 7);
        for _ in 0..5 {
            tmp |= all_side_watch_board & (tmp << 7);
        }
        legal_board |= blank_board & (tmp << 7);

        // lower left
        tmp = all_side_watch_board & (self.player_board >> 7);
        for _ in 0..5 {
            tmp |= all_side_watch_board & (tmp >> 7);
        }
        legal_board |= blank_board & (tmp >> 7);

        // lower right
        tmp = all_side_watch_board & (self.player_board >> 9);
        for _ in 0..5 {
            tmp |= all_side_watch_board & (tmp >> 9);
        }
        legal_board |= blank_board & (tmp >> 9);

        legal_board
    }

    pub fn reverse(&mut self, put: u64) {
        let mut rev = 0;
        for k in 0..8 {
            let mut rev_ = 0;
            let mut mask = self.transfer(put, k);
            while (mask != 0) && (mask & self.opponent_board != 0) {
                rev_ |= mask;
                mask = self.transfer(mask, k);
            }
            if mask & self.player_board != 0 {
                rev |= rev_;
            }
        }

        self.player_board ^= put | rev;
        self.opponent_board ^= rev;
    }

    fn transfer(&self, put: u64, k: u32) -> u64 {
        match k {
            0 => return (put << 8) & 0xffffffffffffff00,
            1 => return (put << 7) & 0x7f7f7f7f7f7f7f00,
            2 => return (put >> 1) & 0x7f7f7f7f7f7f7f7f,
            3 => return (put >> 9) & 0x007f7f7f7f7f7f7f,
            4 => return (put >> 8) & 0x00ffffffffffffff,
            5 => return (put >> 7) & 0x00fefefefefefefe,
            6 => return (put << 1) & 0xfefefefefefefefe,
            7 => return (put << 9) & 0xfefefefefefefe00,
            _ => return 0,
        }
    }

    pub fn is_pass(&self) -> bool {
        self.get_legal_board() == 0
    }

    pub fn is_end(&self) -> bool {
        self.player_board.count_ones() + self.opponent_board.count_ones() == 64
    }

    pub fn get_result(&self) -> (i32, i32) {
        let player = self.player_board.count_ones();
        let opponent = self.opponent_board.count_ones();

        if player > opponent {
            return (player as i32, opponent as i32);
        } else if player < opponent {
            return (opponent as i32, player as i32);
        } else {
            return (player as i32, opponent as i32);
        }
    }

    pub fn swap(&mut self) {
        let tmp = self.player_board;
        self.player_board = self.opponent_board;
        self.opponent_board = tmp;
    }

    // 合法手をランダムにさす
    pub fn random_put(&mut self) {
        let legal_board = self.get_legal_board();
        let mut rng = rand::thread_rng();

        let mut legal_board_vec = Vec::new();
        for i in 0..64 {
            if legal_board & (1 << i) != 0 {
                legal_board_vec.push(i);
            }
        }

        let random_index = rng.gen_range(0..legal_board_vec.len());
        let hand = 1 << legal_board_vec[random_index];

        self.reverse(hand);
    }
}
