use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// 評価値の種類を定義
#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    Exact,      // 評価値が正確 (alpha < score < beta)
    LowerBound, // 評価値は少なくともこの値以上 (score >= beta)
    UpperBound, // 評価値は最大でもこの値 (score <= alpha)
}

// 置換表に格納するデータ
#[derive(Clone, Copy)]
pub struct TableEntry {
    pub score: i32,
    pub depth: usize,
    pub node_type: NodeType,
}

// 置換表本体。スレッドセーフなHashMap。
pub struct TranspositionTable {
    map: Mutex<HashMap<(u64, u64), TableEntry>>,
}

impl TranspositionTable {
    fn new() -> Self {
        TranspositionTable {
            map: Mutex::new(HashMap::new()),
        }
    }

    /// データを格納する
    pub fn store(&self, key: (u64, u64), entry: TableEntry) {
        let mut table = self.map.lock().unwrap();
        // 既に登録されているデータが、今回格納するものより深い探索結果なら上書きしない
        if let Some(existing_entry) = table.get(&key) {
            if existing_entry.depth > entry.depth {
                return;
            }
        }
        table.insert(key, entry);
    }

    /// データを参照する
    pub fn probe(&self, key: (u64, u64)) -> Option<TableEntry> {
        let table = self.map.lock().unwrap();
        table.get(&key).cloned()
    }

    /// テーブルをクリアする
    pub fn clear(&self) {
        let mut table = self.map.lock().unwrap();
        table.clear();
    }
}

// プログラム全体で共有されるグローバルな置換表インスタンス
pub static TT: Lazy<TranspositionTable> = Lazy::new(TranspositionTable::new);
