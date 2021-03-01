// 4 inline w/alpha beta prunning

use std::time::Instant;


mod fourinline;
use fourinline::*;

fn cli_play() {
    const LEVEL : u32 = 9;

    use std::io::{stdin, stdout, Write};

    let mut fil = Fourinline::new();

    let mut line = String::new();
    let mut n_moves = 0;

    '_exit : loop {
        let t = Instant::now();
        
        let res = fil.play(LEVEL);

        n_moves+=1;
        let res = match res {
            MAX_EVAL => "(i'll win)".to_string(),
            MIN_EVAL => "you can win".to_string(),
            _        =>  res.to_string(),
        };

        println!("my move: {}, result: {}, lap: {:.1?}", fil.best_move.col, res, Instant::now() - t);

        fil.board.print();
        
        if fil.computer_wins() {
            println!("i win in {} moves! at level {}", n_moves, LEVEL);
            break '_exit
        }

        if fil.board.is_draw() {
            println!("draw in {} moves", n_moves);
            break '_exit
        }

        loop {
            print!("your move? ");   let _=stdout().flush();

            line.clear();
            stdin().read_line(&mut line).expect("Did not enter a correct string");
            let line = line.chars().nth(0).unwrap();

            if line=='q' { break '_exit }

            let col = line as u32 - '0' as u32;
            if fil.board.move_check( col, Chip::Human ) { 
                if fil.human_wins() {
                    fil.board.print();
                    println!("won in {} moves! at level {}", n_moves, LEVEL);
                    break '_exit
                }            
                break 
            }

            
        }
    }    
}


fn main() {
    cli_play();
}