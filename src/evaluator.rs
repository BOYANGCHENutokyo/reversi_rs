use crate::bitboard::Board;

// 盤面の各マスが持つ静的な価値。隅は価値が高く、その隣は価値が低い。
const POSITION_VALUES: [i32; 64] = [
    120, -20,  20,   5,   5,  20, -20, 120,
    -20, -40,  -5,  -5,  -5,  -5, -40, -20,
     20,  -5,  15,   3,   3,  15,  -5,  20,
      5,  -5,   3,   3,   3,   3,  -5,   5,
      5,  -5,   3,   3,   3,   3,  -5,   5,
     20,  -5,  15,   3,   3,  15,  -5,  20,
    -20, -40,  -5,  -5,  -5,  -5, -40, -20,
    120, -20,  20,   5,   5,  20, -20, 120,
];

/// 盤面を評価し、スコアを返すメイン関数
pub fn evaluate_board(board: &Board, black_moves: u64, white_moves: u64) -> i32 {
    let my_stones = board.get_black();
    let opp_stones = board.get_white();

    // 1. 盤上の石の配置による評価 (隅や辺は重要)
    let position_score = evaluate_positions(my_stones, opp_stones);

    // 2. 着手可能数 (モビリティ) による評価
    let mobility_score = evaluate_mobility(black_moves, white_moves);
    
    // 3. 確定石による評価 (簡易版)
    let confirmed_score = evaluate_confirmed_stones(my_stones, opp_stones);

    // ゲームの進行度 (手数) に応じて、各評価値の重みを変える
    let turn = board.turns;
    if turn < 20 {
        // 序盤は配置と着手可能数を重視
        position_score * 2 + mobility_score * 3 + confirmed_score * 5
    } else if turn < 48 {
        // 中盤は配置、着手可能数、確定石をバランス良く評価
        position_score * 2 + mobility_score * 2 + confirmed_score * 5
    } else {
        // 終盤は石の数を最も重視する (evaluate_endで処理されるため、ここでは確定石を重視)
        position_score + mobility_score + confirmed_score * 10
    }
}

/// 石の配置に基づいてスコアを計算する
fn evaluate_positions(my_stones: u64, opp_stones: u64) -> i32 {
    let mut score = 0;
    for i in 0..64 {
        let pos = 1u64 << i;
        if (my_stones & pos) != 0 {
            score += POSITION_VALUES[i];
        } else if (opp_stones & pos) != 0 {
            score -= POSITION_VALUES[i];
        }
    }
    score
}

/// 着手可能数 (モビリティ) に基づいてスコアを計算する
fn evaluate_mobility(my_moves: u64, opp_moves: u64) -> i32 {
    let my_mobility = my_moves.count_ones() as i32;
    let opp_mobility = opp_moves.count_ones() as i32;
    (my_mobility - opp_mobility) * 10 // 1手あたり10点の価値
}

/// 確定石 (四隅とその隣) に基づいてスコアを計算する (簡易版)
fn evaluate_confirmed_stones(my_stones: u64, opp_stones: u64) -> i32 {
    let mut score = 0;
    let corners = 0x8100000000000081; // 四隅のビットマスク
    
    let my_corners = (my_stones & corners).count_ones() as i32;
    let opp_corners = (opp_stones & corners).count_ones() as i32;

    // 隅は非常に価値が高い
    score += (my_corners - opp_corners) * 200;

    score
}
