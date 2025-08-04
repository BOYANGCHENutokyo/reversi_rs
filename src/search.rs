use rand::random;

use crate::bitboard::Board;

// bookモジュールとグローバルな定石DBをインポート
use crate::book::OPENING_BOOK;

// Import transtable
// use crate::transposition::{TT, TableEntry, NodeType};

// alpha-beta探索
// alpha_beta関数を置換表対応に修正
/*
pub fn alpha_beta(board: &Board, alpha: i32, beta: i32, depth: usize, pass: bool) -> i32 {
    // 引数のalphaは変更せず、変更可能なローカル変数を作成する
    let mut current_alpha = alpha;
    // 置換表への格納時に比較するため、元のalphaの値を保持
    let original_alpha = alpha;

    let key = (board.get_black(), board.get_white());

    // --- 1. 置換表の参照 ---
    if let Some(entry) = TT.probe(key) {
        if entry.depth >= depth {
            match entry.node_type {
                NodeType::Exact => return entry.score,
                NodeType::LowerBound => {
                    if entry.score >= beta { return beta; }
                    if entry.score > current_alpha { current_alpha = entry.score; }
                }
                NodeType::UpperBound => {
                    if entry.score <= current_alpha { return current_alpha; }
                }
            }
        }
    }

    let (black_mvs, hints) = board.legals();

    if black_mvs == 0 && pass {
        return board.evaluate_end();
    } else if black_mvs == 0 {
        let mut new_board = board.clone();
        new_board.exchange();
        return -alpha_beta(&new_board, -beta, -alpha, depth, true);
    } else if depth == 0 {
        // 評価関数の呼び出し方を既存コードに合わせる
        return board.evaluate(black_mvs, board.get_opponent_legals());
    }

    // --- 2. 既存の探索ロジック ---
    let mut mvs = (0..64)
        .map(|i| 1 << i)
        .filter(|&mv| mv & black_mvs == mv)
        .collect::<Vec<_>>();
    let n = mvs.len();
    for i in 0..n - 1 {
        mvs.swap(i, i + random::<usize>() % (n - i));
    }

    let mut best_score = -Board::MAX_EVAL;
    for &mov in mvs.iter() {
        let mut new_board = board.clone();
        new_board.next(mov, hints);
        new_board.exchange();
        // 再帰呼び出しには、更新されたcurrent_alphaを渡す
        let score = -alpha_beta(&new_board, -beta, -current_alpha, depth - 1, false);
        
        if score > best_score {
            best_score = score;
        }
        // ローカル変数のalphaを更新
        if best_score > current_alpha {
            current_alpha = best_score;
        }
        // betaカットの判定
        if current_alpha >= beta {
            break;
        }
    }

    // --- 3. 置換表への格納 ---
    let node_type = if best_score <= original_alpha {
        NodeType::UpperBound
    } else if best_score >= beta {
        NodeType::LowerBound
    } else {
        NodeType::Exact
    };
    TT.store(key, TableEntry { score: best_score, depth, node_type });

    best_score
}
*/

// alpha-beta探索
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
    // 1. まず定石データベースを検索する
    if let Some(book_move) = OPENING_BOOK.get(board) {
        eprintln!("[Info] Move from Opening Book!");
        // 定石手が見つかった場合、既存の返り値の型に合わせて返す（配列の部分はダミーデータで埋める）
        let (_, hints) = board.legals();
        return (book_move, hints);
    }

    // 2. 定石が見つからなかった場合、普通の探索処理を実行する
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
            // 46手以降は読み切りモード
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
