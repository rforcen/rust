// FourInLine

const N : u32 = 7;
const N_COL : u32 = N;
const N_ROW : u32 = N-1;

const EVAL_DRAW : i32 = 2;
pub const MAX_EVAL : i32 = i32::MAX;
pub const MIN_EVAL : i32 = -i32::MAX;

// Chip 

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Chip { Empty, Human, Machine }

// Board

#[derive(Clone, Debug)]
pub struct Board {
    board   : Vec<Chip>,
    cols_sum: Vec<u32>,
}
impl Board {
    fn new() -> Self { Self {
        board   : vec![Chip::Empty ; (N_COL * N_ROW) as usize],
        cols_sum: vec![0; N_COL as usize],
    }}

    fn clear(&mut self) {
        self.board      = vec![Chip::Empty ; (N_COL * N_ROW) as usize];
        self.cols_sum   = vec![0; N_COL as usize];
    }

    fn set(&mut self, row : u32, col : u32, chip : Chip) {
        self.board[(row * N_COL + col) as usize] = chip
    }
    fn get(&self, row : u32, col : u32) -> Chip {
        self.board[(row * N_COL + col) as usize]
    }

    fn generate_moves(&self) -> Vec<u32> {
        let mut moves = vec![];
        for i in 0..N_COL {
            let mut p = i + N_COL/2;
            if p >= N_COL { p -= N_COL }
            if self.cols_sum[p as usize] < N_ROW { moves.push(p) } 
        }
        moves
    }

    pub fn is_draw(&self) -> bool {
        self.generate_moves().len() == 0
    }

    fn _move(&mut self, col : u32, who : Chip) {
        self.set(self.cols_sum[col as usize], col, who);
        self.cols_sum[col as usize]+=1;
    }
    
    pub fn move_check(&mut self, col : u32, who : Chip) -> bool {
        let ok = self.cols_sum[col as usize] < N_ROW && col < N_COL;
        if ok {
            self.set(self.cols_sum[col as usize], col, who);
            self.cols_sum[col as usize]+=1
        }
        ok
    }
    fn take(&mut self, col : u32) {
        self.cols_sum[col as usize]-=1;
        self.set(self.cols_sum[col as usize], col, Chip::Empty);        
    }

    pub fn print(&self) {
        for r in 0..N_ROW {
            for c in 0..N_COL {
                print!("{} ", match self.get((N_ROW-1)-r,c) {
                    Chip::Human     => 'O',
                    Chip::Machine   => 'X',
                    Chip::Empty     => '·',
                });
            }
            print!("\n");
        }
        println!("-------------");
        println!("0 1 2 3 4 5 6");
    }
}

// Move

#[derive(Clone, Debug)]
pub struct Move {
    pub col     : u32,
    res     : i32,
    chip    : Chip,
}
impl Move {
    fn new() -> Self { Self { col:0, res:-1, chip:Chip::Empty} }
    fn set_if_better(&mut self, col:u32, res:i32, chip:Chip) {
        if res > self.res {
            *self = Self{col, res, chip}
        }
    }
    fn set(&mut self, col:u32, res:i32, chip:Chip) {
        *self = Self{col, res, chip}
    }
    fn clear(&mut self) {
        *self = Self::new()
    }
}

// FourinLine

#[derive(Clone, Debug)]
pub struct Fourinline {
    pub board       : Board,
    pub best_move   : Move,
    win_coords  : Vec<Vec<(u32,u32)>>,
}

impl Fourinline {
    pub fn new() -> Self { 
        Self {
            board       : Board::new(), 
            best_move   : Move::new(),
            win_coords  : Self::find_all_winning_coords()} }

      
    fn _move(&mut self, mv : Move) {
        self.board._move(mv.col, mv.chip)
    }

    pub fn play(&mut self, level : u32) -> i32 {
        self.best_move.clear();
        let res = self.alpha_beta(level, level, -i32::MAX, i32::MAX, Chip::Human);
        self._move( self.best_move.clone() );
        res
    }

    fn alpha_beta(&mut self, level : u32, max_level : u32, alpha : i32, beta : i32, who : Chip ) -> i32 {
        let mut board_val = 0;
        let mut eval : i32;

        let (mut alpha, mut beta) = ( alpha, beta );

        if level == 0 {
            board_val = self.evaluate( who ) // eval. terminal node
        } else {
            let moves = self.board.generate_moves();

            if moves.len() > 0 {
                match who {
                    Chip::Human => { // test all computer moves
                        for mv in moves {
                            self.board._move(mv, Chip::Machine);

                            if self.computer_wins() {
                                eval = MAX_EVAL;
                                if level == max_level {
                                    self.best_move.set_if_better(mv, MAX_EVAL, Chip::Machine )
                                }
                            } else {
                                eval = self.alpha_beta(level - 1, max_level, alpha, beta, Chip::Machine)
                            }

                            if eval > alpha {
                                alpha = eval;
                                if level == max_level {
                                    self.best_move.set_if_better(mv, alpha, Chip::Machine)
                                }
                            }

                            self.board.take(mv);

                            if beta <= alpha { break } // beta prune
                        }

                        board_val = alpha
                    }

                    Chip::Machine => { // test all human moves
                        for mv in moves {
                            self.board._move(mv, Chip::Human);

                            if self.human_wins() {
                                eval = -MAX_EVAL;
                                alpha = -MAX_EVAL;
                            } else {
                                eval = self.alpha_beta(level - 1, max_level, alpha, beta, Chip::Human)
                            }

                            if eval < beta {
                                beta = eval;
                                if level == max_level {
                                    self.best_move.set(mv, beta, Chip::Machine)
                                }
                            }

                            self.board.take(mv);

                            if beta <= alpha { break } // alpha prune
                        }

                        board_val = beta
                    }
                    _ => {}
                }

            } else {
                board_val = EVAL_DRAW;
            }
        }

        board_val
    }

    pub fn computer_wins(&self) -> bool {  self.evaluate(Chip::Machine) == MAX_EVAL   }
    pub fn human_wins(&self) -> bool    {  self.evaluate(Chip::Human) == MAX_EVAL    }

    fn evaluate(&self, chip : Chip) -> i32 {
        if self.is_winner(chip) { MAX_EVAL } else {0}
    }

    pub fn is_winner(&self, chip : Chip) -> bool {
        let mut is_winner = false;

        'end: 
        for wcs in &self.win_coords {
    
            let mut is_win = true;
            for c4 in wcs {
                if self.board.get(c4.0, c4.1) != chip { is_win = false; break }
            }
            if is_win { is_winner=true;  break 'end }
        }
        is_winner
    }

    pub fn find_all_winning_coords() -> Vec<Vec<(u32,u32)>> {
    
        let mut win_coords = vec![vec![(0u32, 0u32)]; 0];

        
        for r in 0..N_ROW { // rows
            for c in 0..(N_COL-4+1) { win_coords.push((0..4).map(|p| (r, c+p)).collect()) }}
        
        for c in 0..N_COL { // cols
            for r in 0..(N_ROW-4+1) { win_coords.push((0..4).map(|p| (r+p, c)).collect()) }}
       
        // diag-right & left      
        for (r, cr) in [2, 1, 0, 0, 0, 0].iter().zip([0..4, 0..5, 0..6].iter()) {
            
            for c in cr.clone() {
                let mut cpr = vec![(0u32, 0u32); 0];
                let mut cpl = vec![(0u32, 0u32); 0];

                let mut np=0;
                for p in 0..4 {
                    if r+p >= N_ROW || c+p >= N_COL { break }
                    cpr.push((r+p, c+p));
                    cpl.push((r+p, (N_COL-1)-(c+p)));
                    np += 1;
                }
                if np == 4 { 
                    win_coords.push(cpr);
                    win_coords.push(cpl)
                }
            }
        }

        win_coords
       
    }

}