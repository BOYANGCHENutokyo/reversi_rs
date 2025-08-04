use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::bitboard::Board;

// ユーザーから提供された定石データを文字列定数として埋め込む
const BOOK_DATA: &'static str = r#"
f5
f5d6
f5d6c3g5
f5d6c3g5c6c5
f5d6c3g5c6c5c4b6
f5d6c3g5c6c5c4b6f6f4
f5d6c3g5c6c5c4b6f6f4e6d7
f5d6c3g5c6c5c4b6f6f4e6d7c7g6
f5d6c3g5c6c5c4b6f6f4e6d7c7g6d8b5
f5d6c3g5c6c5c4b6f6f4e6d7c7g6d8b5e7b3
f5d6c3g5c6c5c4b6f6f4e6d7c7g6d8b5e7b3a6e3
f5d6c3g5c6c5c4b6f6f4e6d7c7g6d8b5e7b3a6e3a5d3
f5d6c3g5f6d3
f5d6c3g5f6d3e3c2
f5d6c3g5f6d3e3c2c1e6
f5d6c3g5f6d3e3c2c1e6f4f3
f5d6c3g5f6d3e3c2c1e6f4f3f2g4
f5d6c3g5f6d3e3c2c1e6f4f3f2g4g6d2
f5d6c3g5f6d3e3c2c1e6f4f3f2g4g6d2h3h4
f5d6c3g5f6d3e3c2c1e6f4f3f2g4g6d2h3h4h5f7
f5d6c3g5f6d3e3c2c1e6f4f3f2g4g6d2h3h4h5f7e7g3
f5d6c3g5g6d3
f5d6c3g5g6d3c4e3
f5d6c3g5g6d3c4e3f3b4
f5d6c3g5g6d3c4e3f3b4f6e6
f5d6c3g5g6d3c4e3f3b4f6e6f4g4
f5d6c3g5g6d3c4e3f3b4f6e6f4g4h4h5
f5d6c3g5g6d3c4e3f3b4f6e6f4g4h4h5h6g3
f5d6c3g5g6d3c4e3f3b4f6e6f4g4h4h5h6g3h3f7
f5d6c3g5g6d3c4e3f3b4f6e6f4g4h4h5h6g3h3f7f8c2
f5d6c4b3
f5d6c4b3b4f4
f5d6c4b3b4f4f6g5
f5d6c4b3b4f4f6g5f3e7
f5d6c4b3b4f4f6g5f3e7c5e6
f5d6c4b3b4f4f6g5f3e7c5e6c3g4
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3e3
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3e3f2b6
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3e3f2b6h4d3
f5d6c5b4
f5d6c5b4d7e7
f5d6c5b4d7e7c7d8
f5d6c5b4d7e7c7d8c3d3
f5d6c5b4d7e7c7d8c3d3c4b3
f5d6c5b4d7e7c7d8c3d3c4b3d2e2
f5d6c5b4d7e7c7d8c3d3c4b3d2e2c2e3
f5d6c5b4d7e7c7d8c3d3c4b3d2e2c2e3f4f2
f5d6c5b4d7e7c7d8c3d3c4b3d2e2c2e3f4f2c6b5
f5d6c5b4d7e7c7d8c3d3c4b3d2e2c2e3f4f2c6b5f3c8
f5d6c4
f5d6c4b3b4
f5d6c4b3b4f4f6
f5d6c4b3b4f4f6g5f3
f5d6c4b3b4f4f6g5f3e7c5
f5d6c4b3b4f4f6g5f3e7c5e6c3
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3e3f2
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3e3f2b6h4
f5d6c4b3b4f4f6g5f3e7c5e6c3g4c6g3h3e3f2b6h4d3e2
f5d6c4d3c3
f5d6c4d3c3b3d2
f5d6c4d3c3b3d2e1b5
f5d6c4d3c3b3d2e1b5c5b4
f5d6c4d3c3b3d2e1b5c5b4e3c2
f5d6c4d3c3b3d2e1b5c5b4e3c2a4c6
f5d6c4d3c3b3d2e1b5c5b4e3c2a4c6d1e2
f5d6c4d3c3b3d2e1b5c5b4e3c2a4c6d1e2c7b6
f5d6c4d3c3b3d2e1b5c5b4e3c2a4c6d1e2c7b6f1e6
f5d6c4d3c3b3d2e1b5c5b4e3c2a4c6d1e2c7b6f1e6f3f2
f5d6c4d3c3f4f6
f5d6c4d3c3f4f6f3e6
f5d6c4d3c3f4f6f3e6e7f7
f5d6c4d3c3f4f6f3e6e7f7c5b6
f5d6c4d3c3f4f6f3e6e7f7c5b6g5e3
f5d6c4d3c3f4f6f3e6e7f7c5b6g5e3d7c6
f5d6c4d3c3f4f6f3e6e7f7c5b6g5e3d7c6e2g4
f5d6c4d3c3f4f6f3e6e7f7c5b6g5e3d7c6e2g4h3d2
f5d6c4d3c3f4f6f3e6e7f7c5b6g5e3d7c6e2g4h3d2g3f1
f5d6c4d3c3f4f6g6e3
f5d6c4d3c3f4f6g6e3e2f1
f5d6c4d3c3f4f6g6e3e2f1d1g5
f5d6c4d3c3f4f6g6e3e2f1d1g5c6d8
f5d6c4d3c3f4f6g6e3e2f1d1g5c6d8g4h6
f5d6c4d3c3f4f6b4c2
f5d6c4d3c3f4f6b4c2f3e3
f5d6c4d3c3f4f6b4c2f3e3e2c6
f5d6c4d3c3f4f6b4c2f3e3e2c6f2c5
f5d6c4d3c3f4f6b4c2f3e3e2c6f2c5e6d2
f5d6c4d3c3f4f6b4c2f3e3e2c6f2c5e6d2g4d7
f5d6c4d3c3f4f6b4c2f3e3e2c6f2c5e6d2g4d7b3g5
f5d6c4d3c3f4f6b4c2f3e3e2c6f2c5e6d2g4d7b3g5c8h4
f5d6c4d3c3f4f6g5e3
f5d6c4d3c3f4f6g5e3f3g6
f5d6c4d3c3f4f6g5e3f3g6e2h5
f5d6c4d3c3f4f6g5e3f3g6e2h5c5g4
f5d6c4d3c3f4f6g5e3f3g6e2h5c5g4g3f2
f5d6c4d3c3b5b4
f5d6c4d3c3b5b4f4c5
f5d6c4d3c3b5b4f4c5a4b3
f5d6c4d3c3b5b4f4c5a4b3d2a6
f5d6c4d3c3b5b4f4c5a4b3d2a6a3e3
f5d6c4d3c3b5b4f4c5a4b3d2a6a3e3f3g4
f5d6c4d3c3b5b4f4c5a4b3d2a6a3e3f3g4e6f6
f5d6c4d3c3b5b4f4c5a4b3d2a6a3e3f3g4e6f6g3e2
f5d6c4d3c3b5b4f4c5a4b3d2a6a3e3f3g4e6f6g3e2c2f2
f5d6c4g5f6
f5d6c4g5f6f4f3
f5d6c4g5f6f4f3d3c3
f5d6c4g5f6f4f3d3c3g6e3
f5d6c4g5f6f4f3d3c3g6e3e6h5
f5d6c4g5f6f4f3d3c3g6e3e6h5d2e2
f5d6c4g5f6f4f3d3c3g6e3e6h5d2e2c2c6
f5d6c4g5f6f4f3d3c3g6e3e6h5d2e2c2c6c5b6
f5d6c4g5f6f4f3d3c3g6e3e6h5d2e2c2c6c5b6b4b3
f5d6c4g5f6f4f3d3c3g6e3e6h5d2e2c2c6c5b6b4b3c7a4
f5f6e6
f5f6e6f4g6
f5f6e6f4g6c5f3
f5f6e6f4g6c5f3g4e3
f5f6e6f4g6c5f3g4e3d6g5
f5f6e6f4g6c5f3g4e3d6g5g3c3
f5f6e6f4g6c5f3g4e3d6g5g3c3h5c4
f5f6e6f4g6c5f3g4e3d6g5g3c3h5c4d7h6
f5f6e6f4g6c5f3g4e3d6g5g3c3h5c4d7h6h7h3
f5f6e6f4g6c5f3g4e3d6g5g3c3h5c4d7h6h7h3f7e7
f5f6e6f4g6c5f3g4e3d6g5g3c3h5c4d7h6h7h3f7e7f8h4
f5f6e6f4g6c5f3g5d6
f5f6e6f4g6c5f3g5d6e3h4
f5f6e6f4g6c5f3g5d6e3h4g3g4
f5f6e6f4g6c5f3g5d6e3h4g3g4h6e2
f5f6e6f4g6c5f3g5d6e3h4g3g4h6e2d3h5
f5f6e6f4g6c5f3g5d6e3h4g3g4h6e2d3h5h3c6
f5f6e6f4g6c5f3g5d6e3h4g3g4h6e2d3h5h3c6e7f2
f5f6e6f4g6c5f3g5d6e3h4g3g4h6e2d3h5h3c6e7f2c4d2
f5f6e6f4g6d6g4
f5f6e6f4g6d6g4g5h4
f5f6e6f4g6d6g4g5h4e7f3
f5f6e6f4g6d6g4g5h4e7f3h6f7
f5f6e6f4g6d6g4g5h4e7f3h6f7e8f8
f5f6e6f4g6d6g4g5h4e7f3h6f7e8f8g8d3
f5f6e6f4g6d6g4g5h4e7f3h6f7e8f8g8d3h5h7
f5f6e6f4g6d6g4g5h4e7f3h6f7e8f8g8d3h5h7e3c5
f5f6e6f4g6d6g4g5h4e7f3h6f7e8f8g8d3h5h7e3c5c4g3
f5f6e6d6f7
f5f6e6d6f7e3c6
f5f6e6d6f7e3c6e7f4
f5f6e6d6f7e3c6e7f4c5d8
f5f6e6d6f7e3c6e7f4c5d8c7d7
f5f6e6d6f7e3c6e7f4c5d8c7d7f8b5
f5f6e6d6f7e3c6e7f4c5d8c7d7f8b5c4e8
f5f6e6d6f7e3c6e7f4c5d8c7d7f8b5c4e8c8f3
f5f6e6d6f7e3c6e7f4c5d8c7d7f8b5c4e8c8f3g5b6
f5f6e6d6f7e3c6e7f4c5d8c7d7f8b5c4e8c8f3g5b6d3b4
f5f6e6d6f7f4d7
f5f6e6d6f7f4d7e7d8
f5f6e6d6f7f4d7e7d8g5c6
f5f6e6d6f7f4d7e7d8g5c6f8g6
f5f6e6d6f7f4d7e7d8g5c6f8g6h5h6
f5f6e6d6f7f4d7e7d8g5c6f8g6h5h6h7c4
f5f6e6d6f7f4d7e7d8g5c6f8g6h5h6h7c4e8g8
f5f6e6d6f7f4d7e7d8g5c6f8g6h5h6h7c4e8g8c5e3
f5f6e6d6f7f4d7e7d8g5c6f8g6h5h6h7c4e8g8c5e3d3c7
"#;

// Lazy<T> を使って、最初にアクセスされたときに一度だけBookを初期化する
// これにより、プログラム全体で共有されるグローバルな定石DBが実現される
pub static OPENING_BOOK: Lazy<Book> = Lazy::new(|| Book::new(BOOK_DATA));

// 定石データベースを表す構造体
// キー: (自分の石のビットボード, 相手の石のビットボード)
// 値: 次に指すべき手を表すビットボード (u64)
pub struct Book {
    map: HashMap<(u64, u64), u64>,
}

impl Book {
    // 定石文字列から新しいBookインスタンスを作成する
    pub fn new(book_data: &'static str) -> Self {
        let mut map = HashMap::new();

        for line in book_data.lines() {
            let line = line.trim();
            // 空行やコメントは無視
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut board = Board::new();
            
            // 2文字ずつ手としてパースしていく (例: "f5d6c3...")
            for i in (0..line.len()).step_by(2) {
                if i + 2 > line.len() { continue; }
                let move_str = &line[i..i + 2];
                
                // 現在の盤面をキーとして保存
                let key = (board.get_black(), board.get_white());

                // 文字列から座標に変換し、さらにビットボードでの表現に変換
                let move_bit = move_str_to_bit(move_str);

                // まだ登録されていなければ、現在の局面と次の手を登録
                if !map.contains_key(&key) {
                    map.insert(key, move_bit);
                }

                // 実際に手を打って盤面を進める
                let (legals, hints) = board.legals();

                if (legals & move_bit) != 0 {
                    board.next(move_bit, hints);
                    board.exchange(); // 手番を交代
                } else {
                    // 定石データに不正な手が含まれている場合は警告を出して中断
                    // eprintln!("Invalid move '{}' in opening book for line: {}", move_str, line);
                    break;
                }
            }
        }
        Book { map }
    }

    // 現在の盤面(board)に一致する定石手があれば返す
    pub fn get(&self, board: &Board) -> Option<u64> {
        let key = (board.get_black(), board.get_white());
        self.map.get(&key).cloned()
    }
}

// "f5" のような文字列を u64 のビットボードに変換する
fn move_str_to_bit(s: &str) -> u64 {
    let mut chars = s.chars();
    let col = match chars.next() {
        Some(c) => c as u8 - b'a',
        None => return 0,
    };
    let row = match chars.next() {
        Some(c) => c as u8 - b'1',
        None => return 0,
    };
    1u64 << (row * 8 + col)
}
