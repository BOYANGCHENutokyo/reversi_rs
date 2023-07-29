use rand::random;

use crate::bitboard::Board;

pub fn alpha_beta(board: &Board, alpha: i32, beta: i32, depth: usize, pass: bool) -> i32 {
    let (black_mvs, hints) = board.legals();
    if black_mvs == 0 && pass {
        // double pass
        board.evaluate_end()
    } else if black_mvs == 0 {
        let mut new_board = board.clone();
        new_board.exchange();
        -alpha_beta(&new_board, -beta, -alpha, depth, true)
    } else if depth == 0 {
        let mut new_board = board.clone();
        new_board.exchange();
        let (white_mvs, _) = new_board.legals();
        board.evaluate(black_mvs, white_mvs)
    } else {
        let mut mvs = (0..64)
            .map(|i| 1 << i)
            .filter(|&mv| mv & black_mvs == mv)
            .collect::<Vec<_>>();
        let n = mvs.len();
        for i in 0..n - 1 {
            mvs.swap(i, i + random::<usize>() % (n - i));
        }
        let mut alpha = alpha;
        for &mov in mvs.iter() {
            let mut new_board = board.clone();
            new_board.next(mov, hints);
            new_board.exchange();
            let score = -alpha_beta(&new_board, -beta, -alpha, depth - 1, false);
            if alpha < score {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }
        alpha
    }
}

pub fn search(board: &Board, depth: usize, time_level: usize) -> (u64, [(u64, u64); 4]) {
    let (mvs, hints) = board.legals();
    if mvs == 0 {
        (0, hints)
    } else {
        let mut mvs = (0..64)
            .map(|i| 1 << i)
            .filter(|&mv| mv & mvs == mv)
            .collect::<Vec<_>>();
        let n = mvs.len();
        for i in 0..n - 1 {
            mvs.swap(i, i + random::<usize>() % (n - i));
        }
        let mut sel_mov = mvs[0];
        let mut alpha = -Board::MAX_EVAL;
        let beta = Board::MAX_EVAL;
        for &mov in mvs.iter() {
            let mut new_board = board.clone();
            new_board.next(mov, hints);
            new_board.exchange();
            let score = if new_board.turns > 46 {
                -alpha_beta(&new_board, -beta, -alpha, 64, false)
            } else {
                -alpha_beta(&new_board, -beta, -alpha, depth - time_level * 2, false)
            };
            if alpha < score {
                alpha = score;
                sel_mov = mov;
            }
            if alpha >= beta {
                break;
            }
        }
        (sel_mov, hints)
    }
}
