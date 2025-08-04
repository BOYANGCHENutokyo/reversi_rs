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

// 各評価要素の重み
const POSITION_WEIGHT: i32 = 1;
const MOBILITY_WEIGHT: i32 = 60;
const CORNER_WEIGHT: i32 = 800;

/// 盤面を評価し、スコアを返すメイン関数
/// 高速化のため、分岐をなくしループを最適化
pub fn evaluate_board(board: &Board, black_moves: u64, white_moves: u64) -> i32 {
    let my_stones = board.get_black();
    let opp_stones = board.get_white();

    // 各評価要素を計算
    let position_score = evaluate_positions(my_stones, opp_stones);
    let mobility_score = evaluate_mobility(black_moves, white_moves);
    let corner_score = evaluate_corners(my_stones, opp_stones);

    // 固定された重み付けで最終スコアを計算。分岐を減らし高速化。
    position_score * POSITION_WEIGHT
        + mobility_score * MOBILITY_WEIGHT
        + corner_score * CORNER_WEIGHT
}

/// 石の配置に基づいてスコアを計算する (高速版)
/// 盤上の石の数だけループするため、64回固定のループより高速。
fn evaluate_positions(mut my_stones: u64, mut opp_stones: u64) -> i32 {
    let mut score = 0;
    // 自分の石のループ
    while my_stones != 0 {
        let index = my_stones.trailing_zeros() as usize;
        score += POSITION_VALUES[index];
        my_stones &= my_stones - 1; // 処理済みのビットを消す
    }
    // 相手の石のループ
    while opp_stones != 0 {
        let index = opp_stones.trailing_zeros() as usize;
        score -= POSITION_VALUES[index];
        opp_stones &= opp_stones - 1; // 処理済みのビットを消す
    }
    score
}

/// 着手可能数 (モビリティ) に基づいてスコアを計算する
fn evaluate_mobility(my_moves: u64, opp_moves: u64) -> i32 {
    let my_mobility = my_moves.count_ones() as i32;
    let opp_mobility = opp_moves.count_ones() as i32;
    my_mobility - opp_mobility
}

/// 四隅の石に基づいてスコアを計算する
fn evaluate_corners(my_stones: u64, opp_stones: u64) -> i32 {
    let corners = 0x8100000000000081; // 四隅のビットマスク
    
    let my_corners = (my_stones & corners).count_ones() as i32;
    let opp_corners = (opp_stones & corners).count_ones() as i32;

    my_corners - opp_corners
}
